package org.odict.models;

import com.fasterxml.jackson.annotation.JsonInclude;

import java.util.ArrayList;
import java.util.List;

@JsonInclude(JsonInclude.Include.NON_NULL)
public class Group {

  private String id;

  private String description;

  private List<String> definitions;

  Group(schema.Group buffer) {
    this.id = buffer.id();

    this.description = buffer.description();

    this.definitions = new ArrayList<>();

    for (int i = 0; i < buffer.definitionsLength(); i++) {
      this.definitions.add(buffer.definitions(i));
    }
  }

  public String getID() {
    return id;
  }

  public String getDescription() {
    return description;
  }

  public List<String> getDefinitions() {
    return definitions;
  }
}
