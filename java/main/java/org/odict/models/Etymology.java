package org.odict.models;

import com.fasterxml.jackson.annotation.JsonInclude;

import java.util.ArrayList;
import java.util.List;

@JsonInclude(JsonInclude.Include.NON_NULL)
public class Etymology {

  private String id;

  private List<Usage> usages;

  private String description;

  Etymology(schema.Etymology buffer) {
    this.id = buffer.id();

    this.description = buffer.description().trim();

    this.usages = new ArrayList<>();

    for (int i = 0; i < buffer.usagesLength(); i++) {
      this.usages.add(new Usage(buffer.usages(i)));
    }
  }

  public List<Usage> getUsages() {
    return usages;
  }

  public String getDescription() {
    return description;
  }
}
