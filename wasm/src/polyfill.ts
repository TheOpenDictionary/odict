export async function polyfill() {
  if (!globalThis.crypto) {
    const crypto = await import("crypto");
    Object.defineProperty(globalThis, "crypto", {
      value: crypto.webcrypto
        ? crypto.webcrypto
        : {
            getRandomValues(b: Buffer) {
              return crypto.randomFillSync(b);
            },
          },
    });
  }
}
