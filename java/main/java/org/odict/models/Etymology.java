package org.odict.models;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import com.fasterxml.jackson.databind.ser.std.MapSerializer;

import java.util.*;

@JsonInclude(JsonInclude.Include.NON_EMPTY)
public class Etymology {

  private String id;

  private Map<String, List<Usage>> usages;

  private String description;

  Etymology(schema.Etymology buffer) {
    this.id = buffer.id();

    this.description = buffer.description().trim();

    this.usages = new HashMap<>();

    for (int i = 0; i < buffer.usagesLength(); i++) {
      Usage usage = new Usage(buffer.usages(i));
      this.usages.computeIfAbsent(usage.getPOS(), k -> new ArrayList<>()).add(usage);
    }
  }

  public String getId() {
    return id;
  }

  public Map<String, List<Usage>> getUsages() {
    return usages;
  }

  public String getDescription() {
    return description;
  }
}
