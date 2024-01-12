CREATE TABLE dictionaries (
    id UUID NOT NULL
    ,name TEXT NOT NULL

    ,CONSTRAINT dictionaries_id_pkey PRIMARY KEY (id)
);

CREATE TABLE entries (
    id UUID NOT NULL
    ,created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    ,updated_at TIMESTAMPTZ
    ,term TEXT NOT NULL
    ,dictionary_id UUID

    ,CONSTRAINT entries_id_pkey PRIMARY KEY (id)
);

CREATE TABLE etymologies (
    id UUID NOT NULL
    ,description TEXT
    ,entry_id UUID NOT NULL

    ,CONSTRAINT etymologies_id_pkey PRIMARY KEY (id)
);

CREATE TABLE senses (
    id UUID NOT NULL
    ,etymology_id UUID NOT NULL

    ,CONSTRAINT senses_id_pkey PRIMARY KEY (id)
);

CREATE TABLE groups (
    id UUID NOT NULL
    ,description TEXT
    ,sense_id UUID NOT NULL

    ,CONSTRAINT groups_id_pkey PRIMARY KEY (id)
);

CREATE TABLE definitions (
    id UUID NOT NULL
    ,value TEXT NOT NULL
    ,sense_id UUID
    ,group_id UUID

    ,CONSTRAINT definitions_id_pkey PRIMARY KEY (id)
);

CREATE TABLE examples (
    id TEXT NOT NULL
    ,text TEXT NOT NULL
    ,definition_id UUID
    ,note_id UUID

    ,CONSTRAINT examples_id_pkey PRIMARY KEY (id)
);

CREATE TABLE notes (
    id UUID NOT NULL
    ,value TEXT
    ,definition_id UUID NOT NULL

    ,CONSTRAINT notes_id_pkey PRIMARY KEY (id)
);

ALTER TABLE entries ADD CONSTRAINT entries_dictionary_id_fkey FOREIGN KEY (dictionary_id) REFERENCES dictionaries (id);

ALTER TABLE etymologies ADD CONSTRAINT etymologies_entry_id_fkey FOREIGN KEY (entry_id) REFERENCES entries (id);

ALTER TABLE senses ADD CONSTRAINT senses_etymology_id_fkey FOREIGN KEY (etymology_id) REFERENCES etymologies (id);

ALTER TABLE groups ADD CONSTRAINT groups_sense_id_fkey FOREIGN KEY (sense_id) REFERENCES senses (id);

ALTER TABLE definitions ADD CONSTRAINT definitions_sense_id_fkey FOREIGN KEY (sense_id) REFERENCES senses (id);

ALTER TABLE definitions ADD CONSTRAINT definitions_group_id_fkey FOREIGN KEY (group_id) REFERENCES groups (id);

ALTER TABLE examples ADD CONSTRAINT examples_definition_id_fkey FOREIGN KEY (definition_id) REFERENCES definitions (id);

ALTER TABLE examples ADD CONSTRAINT examples_note_id_fkey FOREIGN KEY (note_id) REFERENCES definitions (id);

ALTER TABLE notes ADD CONSTRAINT notes_definition_id_fkey FOREIGN KEY (definition_id) REFERENCES definitions (id);
;

INSERT INTO dictionaries (name, id) VALUES ('Example Dictionary 1', 'c808b2e0-0adf-4838-815e-55ae09f64ee0');
INSERT INTO entries (id, term, dictionary_id) VALUES ('72032dfe-6f6b-4161-b56f-f1fc20de8f89', 'dog', 'c808b2e0-0adf-4838-815e-55ae09f64ee0');
INSERT INTO etymologies (id, description, entry_id) VALUES ('eba3cf89-2ed0-4f5f-9229-d3a684738063', 'Latin root', '72032dfe-6f6b-4161-b56f-f1fc20de8f89');
INSERT INTO senses (id, etymology_id) VALUES ('e732da82-12ca-4328-9b54-5b3cab8d058e', 'eba3cf89-2ed0-4f5f-9229-d3a684738063');
INSERT INTO definitions (id, value, sense_id) VALUES ('f949e6a1-17df-4be7-9a44-c8dc42aa4754', 'a dog', 'e732da82-12ca-4328-9b54-5b3cab8d058e');
INSERT INTO entries (id, term, dictionary_id) VALUES ('842e0667-31d6-4986-8bcf-091409be4d96', 'poo', 'c808b2e0-0adf-4838-815e-55ae09f64ee0');
INSERT INTO etymologies (id, description, entry_id) VALUES ('4d92460f-fc8c-47ef-97b1-f5fe7fc79655', 'Latin root', '842e0667-31d6-4986-8bcf-091409be4d96');
INSERT INTO senses (id, etymology_id) VALUES ('e950da2d-6b91-47e7-ac91-560c79585a6e', '4d92460f-fc8c-47ef-97b1-f5fe7fc79655');
INSERT INTO groups (id, description, sense_id) VALUES ('0afc99b9-7f08-4715-a22b-6846e4f31b1e', 'A number of verb senses', 'e950da2d-6b91-47e7-ac91-560c79585a6e');
INSERT INTO definitions (id, value) VALUES ('18e72cde-ede2-4b06-aac5-6edb3a6385d1', 'crap, shit');
INSERT INTO entries (id, term, dictionary_id) VALUES ('68449651-8ead-4a86-a42c-f0c5b3b5fd25', 'ran', 'c808b2e0-0adf-4838-815e-55ae09f64ee0');
INSERT INTO entries (id, term, dictionary_id) VALUES ('91760385-5dd5-4fa6-955e-ee3d923de51f', 'run', 'c808b2e0-0adf-4838-815e-55ae09f64ee0');
INSERT INTO etymologies (id, description, entry_id) VALUES ('b47d9d9e-9ba8-447b-b25b-221ba2bc57d4', 'Latin root', '91760385-5dd5-4fa6-955e-ee3d923de51f');
INSERT INTO senses (id, etymology_id) VALUES ('1eb17d2f-0d81-4591-ba0c-5a19afcf3947', 'b47d9d9e-9ba8-447b-b25b-221ba2bc57d4');
INSERT INTO definitions (id, value, sense_id) VALUES ('562514d9-0028-436e-8999-906573dc38f1', 'A group of fish that migrate, or ascend a river for the purpose of spawning.', '1eb17d2f-0d81-4591-ba0c-5a19afcf3947');
INSERT INTO examples (id, text, definition_id) VALUES ('3b49607e-66c1-43ca-8d1b-73b1f169ac7c', 'The horse ran away.', '562514d9-0028-436e-8999-906573dc38f1');
INSERT INTO examples (id, text, definition_id) VALUES ('8410b975-4e5c-4979-9479-8945d87665cf', 'The horse ran away.', '562514d9-0028-436e-8999-906573dc38f1');
INSERT INTO definitions (id, value, sense_id) VALUES ('dc281764-8032-4c87-82fe-f692d02e3c25', 'A group of fish that migrate, or ascend a river for the purpose of spawning.', '1eb17d2f-0d81-4591-ba0c-5a19afcf3947');
INSERT INTO definitions (id, value, sense_id) VALUES ('6b7f6280-5b82-4afa-95dc-234bfbc1e836', 'A group of fish that migrate, or ascend a river for the purpose of spawning.', '1eb17d2f-0d81-4591-ba0c-5a19afcf3947');
INSERT INTO definitions (id, value, sense_id) VALUES ('d5064e42-1155-4c32-8649-dc77e538b03f', 'A group of fish that migrate, or ascend a river for the purpose of spawning.', '1eb17d2f-0d81-4591-ba0c-5a19afcf3947');
INSERT INTO definitions (id, value, sense_id) VALUES ('6681a2bd-764f-4355-8feb-ad25823960ac', 'A group of fish that migrate, or ascend a river for the purpose of spawning.', '1eb17d2f-0d81-4591-ba0c-5a19afcf3947');
INSERT INTO definitions (id, value, sense_id) VALUES ('13397ec1-0fc8-47f9-95ea-89b18a462866', 'A group of fish that migrate, or ascend a river for the purpose of spawning.', '1eb17d2f-0d81-4591-ba0c-5a19afcf3947');
INSERT INTO senses (id, etymology_id) VALUES ('643a4089-7a3b-40e2-a15d-a5671e6956e1', 'b47d9d9e-9ba8-447b-b25b-221ba2bc57d4');
INSERT INTO groups (id, description, sense_id) VALUES ('34187acf-a845-46c4-908b-9d0bb178911f', 'A number of verb senses', '643a4089-7a3b-40e2-a15d-a5671e6956e1');
INSERT INTO definitions (id, value) VALUES ('49b50cb2-3070-418f-8a47-cc569bf40aa5', '(transitive) To execute or carry out a plan, procedure or program.');
INSERT INTO examples (id, text, definition_id) VALUES ('0cad8499-716d-40a0-8ac8-24312e5b519b', 'The horse ran away.', '49b50cb2-3070-418f-8a47-cc569bf40aa5');
INSERT INTO examples (id, text, definition_id) VALUES ('8ca5aa61-2455-458f-8ffd-d311109cdbe7', 'The horse ran away.', '49b50cb2-3070-418f-8a47-cc569bf40aa5');
INSERT INTO definitions (id, value) VALUES ('4813040f-19f4-42f9-b028-967698341f2b', '(transitive) To execute or carry out a plan, procedure or program.');
INSERT INTO definitions (id, value) VALUES ('60ca7f5d-1604-4fb8-a47e-4fc042bf2ebc', '(transitive) To execute or carry out a plan, procedure or program.');
INSERT INTO definitions (id, value) VALUES ('ecb94985-8f40-420d-842d-af56c9e53c67', '(transitive) To execute or carry out a plan, procedure or program.');
INSERT INTO definitions (id, value) VALUES ('8514a307-7474-4d96-a540-4c5bba50c275', '(transitive) To execute or carry out a plan, procedure or program.');
INSERT INTO definitions (id, value) VALUES ('52b3c227-21d9-4b0b-88b4-4ab1fbadba4e', '(transitive) To execute or carry out a plan, procedure or program.');
INSERT INTO entries (id, term, dictionary_id) VALUES ('c0ea27a7-933c-417c-8d5e-4a5f254eb857', 'cat', 'c808b2e0-0adf-4838-815e-55ae09f64ee0');
INSERT INTO etymologies (id, description, entry_id) VALUES ('c87a821c-3e78-4630-9571-2c50605d4808', 'Latin root', 'c0ea27a7-933c-417c-8d5e-4a5f254eb857');
INSERT INTO senses (id, etymology_id) VALUES ('b7a855e1-2dfe-4464-8027-8db49e8569f4', 'c87a821c-3e78-4630-9571-2c50605d4808');
INSERT INTO definitions (id, value, sense_id) VALUES ('db8b41a3-af2a-44d0-bbbc-8d336ec75423', 'a cat', 'b7a855e1-2dfe-4464-8027-8db49e8569f4');
INSERT INTO notes (id, value, definition_id) VALUES ('2c3aee47-a527-4b4c-a2e9-b3587b63f4e1', 'Some definition note', 'db8b41a3-af2a-44d0-bbbc-8d336ec75423');
INSERT INTO examples (id, text) VALUES ('65e85846-7b03-4245-ac85-d24a527cd666', 'Some example');
INSERT INTO examples (id, text, definition_id) VALUES ('44950e9d-fca1-4ab7-89b6-df264c72167d', 'There goes a cat!', 'db8b41a3-af2a-44d0-bbbc-8d336ec75423')