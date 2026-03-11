def test_to_json(dict1):
    json_str = dict1.to_json(pretty=False)
    assert isinstance(json_str, str)
    assert '"term":"cat"' in json_str

def test_to_html(dict1):
    html_str = dict1.to_html()
    assert isinstance(html_str, str)
    assert "cat" in html_str

def test_to_markdown(dict1):
    md_str = dict1.to_markdown()
    assert isinstance(md_str, str)
    assert "cat" in md_str
