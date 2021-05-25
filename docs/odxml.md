# ODXML

ODict XML is the XML variant used to define and structure ODict dictionaries. All compiled ODict dictionaries originate from ODXML files, and can easily be reverted back to XML via the CLI [`dump` command](cli.md#dumping-dictionaries).

An example dictionary might look like this:

```xml
<!-- Dictionary Root -->
<dictionary name="My Dictionary">
  <!-- Entry -->
  <entry term="Doggo">
    <!-- Etymology -->
    <ety>
      <!-- Usage (typically determined by part-of-speech) -->
      <usage pos="n">
        <!-- Definition -->
        <definition>A cute pupper</definition>
        <!-- Definition Group -->
        <group description="Slang for dog">
          <definition>Common way of saying a dog is a cutie</definition>
        </group>
      </usage>
    </ety>
  </entry>
</dictionary>
```

Pretty easy to read, right?

Now let's break this down.

---

## `<definition>`

Dictionary nodes occur at the base of all source files and will not compile without one. ODict looks for these nodes by default when compiling.

### Attributes

| Name | Description                           | Required? |
| ---- | ------------------------------------- | --------- |
| name | A descriptive name for the dictionary | :x:       |

### Children

- [`<entry>`](#entry)

---

## `<entry>`

Entries are the primary entry point to the dictionary and represent **unique terms**. They are used as lookup keys internally by ODict, so it is important there are no duplicate entries.

### Attributes

| Name | Description                   | Required?          |
| ---- | ----------------------------- | ------------------ |
| term | The word the entry represents | :white_check_mark: |

### Children

- [`<entry>`](#entry)
