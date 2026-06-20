import type { APIRoute } from "astro";

const releasesUrl = "https://api.github.com/repos/TheOpenDictionary/odict/releases";
const cliTagPrefix = "cli/";
const installerAssetName = "odict-cli-installer.sh";
const githubToken = import.meta.env.GITHUB_TOKEN as string | undefined;

type GitHubReleaseAsset = {
  name?: string;
  url?: string;
};

type GitHubRelease = {
  tag_name?: string;
  draft?: boolean;
  prerelease?: boolean;
  assets?: GitHubReleaseAsset[];
};

const githubHeaders = {
  Accept: "application/vnd.github+json",
  "User-Agent": "odict-docs-install-endpoint",
  "X-GitHub-Api-Version": "2022-11-28",
  ...(githubToken ? { Authorization: `Bearer ${githubToken}` } : {}),
};

const shellHeaders = {
  "Cache-Control": "public, max-age=300",
  "Content-Type": "text/x-shellscript; charset=utf-8",
};

const textHeaders = {
  "Content-Type": "text/plain; charset=utf-8",
};

const errorResponse = (message: string, status: number) =>
  new Response(message, {
    status,
    headers: textHeaders,
  });

const fetchLatestCliRelease = async () => {
  for (let page = 1; page <= 10; page += 1) {
    const response = await fetch(`${releasesUrl}?per_page=100&page=${page}`, {
      headers: githubHeaders,
    });

    if (!response.ok) {
      throw new Error(`GitHub releases request failed: ${response.status} ${response.statusText}`);
    }

    const releases = (await response.json()) as GitHubRelease[];

    if (!Array.isArray(releases) || releases.length === 0) {
      return undefined;
    }

    const release = releases.find(
      ({ draft, prerelease, tag_name }) =>
        !draft && !prerelease && tag_name?.startsWith(cliTagPrefix),
    );

    if (release) {
      return release;
    }
  }

  return undefined;
};

export const GET: APIRoute = async () => {
  try {
    const release = await fetchLatestCliRelease();

    if (!release) {
      return errorResponse("No CLI release found.", 404);
    }

    const installerAsset = release.assets?.find(({ name }) => name === installerAssetName);

    if (!installerAsset?.url) {
      return errorResponse(
        `Release ${release.tag_name ?? "(unknown)"} does not include ${installerAssetName}.`,
        404,
      );
    }

    const script = await fetch(installerAsset.url, {
      headers: {
        ...githubHeaders,
        Accept: "application/octet-stream",
      },
    });

    if (!script.ok) {
      return errorResponse(`Installer download failed: ${script.status} ${script.statusText}`, 502);
    }

    return new Response(await script.text(), {
      headers: shellHeaders,
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : "Unexpected installer failure.";

    return errorResponse(message, 502);
  }
};
