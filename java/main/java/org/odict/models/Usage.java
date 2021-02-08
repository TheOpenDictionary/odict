package org.odict.models;

import com.fasterxml.jackson.annotation.JsonInclude;

import java.util.ArrayList;
import java.util.List;

@JsonInclude(JsonInclude.Include.NON_NULL)
public class Usage {

  private String pos;

  private List<Group> groups;

  private List<String> definitions;

  Usage(schema.Usage buffer) {
    this.pos = schema.POS.name(buffer.pos());

    this.groups = new ArrayList<>();

    this.definitions = new ArrayList<>();

    for (int i = 0; i < buffer.groupsLength(); i++) {
      this.groups.add(new Group(buffer.groups(i)));
    }

    for (int i = 0; i < buffer.definitionsLength(); i++) {
      this.definitions.add(buffer.definitions(i));
    }
  }

  public String getPOS() {
    return pos;
  }

  public List<Group> getGroups() {
    return groups;
  }

  public List<String> getDefinitions() {
    return definitions;
  }
}
