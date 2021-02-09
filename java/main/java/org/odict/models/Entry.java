package org.odict.models;

import com.fasterxml.jackson.annotation.JsonInclude;

import java.util.ArrayList;
import java.util.List;

@JsonInclude(JsonInclude.Include.NON_EMPTY)
public class Entry {

  String term;

  List<Etymology> etymologies;

  public Entry(schema.Entry buffer) {
    this.term = buffer.term();

    this.etymologies = new ArrayList<>();

    for (int i = 0; i < buffer.etymologiesLength(); i++) {
      this.etymologies.add(new Etymology(buffer.etymologies(i)));
    }
  }

  public String getTerm() {
    return this.term;
  }

  public List<Etymology> getEtymologies() {
    return etymologies;
  }
}
