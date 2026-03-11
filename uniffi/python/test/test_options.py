from odict_uniffi import RemoteLoadOptions, LoadOptions

def test_load_options_structure():
    remote = RemoteLoadOptions(out_dir="/tmp", caching=True, retries=3)
    opts = LoadOptions(config_dir="/etc/odict", remote=remote)
    assert opts.config_dir == "/etc/odict"
    assert opts.remote.out_dir == "/tmp"
    assert opts.remote.caching is True
    assert opts.remote.retries == 3
