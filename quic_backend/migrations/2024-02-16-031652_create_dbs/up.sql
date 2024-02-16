-- datetime like this:
-- let now = select(diesel::dsl::now).get_result::<NaiveDateTime>(conn)?;

-- Your SQL goes here
PRAGMA foreign_keys = ON;

-- The main entry point for a template. 
CREATE TABLE IF NOT EXISTS Template (
  t_id INTEGER NOT NULL PRIMARY KEY,
  -- name in localize table. 
  uuid TEXT NOT NULL UNIQUE,
  -- description in localize table. 
  -- stages in Stage. 
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- The main entry point for single stage. 
CREATE TABLE IF NOT EXISTS Stage (
  s_id INTEGER NOT NULL PRIMARY KEY,
  step INTEGER NOT NULL,
  -- name in localize table
  -- pipeline in Reminders, etc. 
  t_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (t_id) REFERENCES Template (t_id),
  CONSTRAINT unq UNIQUE (step, t_id)
);

-- This table has no link. 
CREATE TABLE IF NOT EXISTS Localize (
  locale_id INTEGER NOT NULL PRIMARY KEY,
  table_name VARCHAR NOT NULL,
  foreign_id INTEGER NOT NULL,  -- No link since it can be from any table. 
  locale VARCHAR NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT unq UNIQUE (table_name, foreign_id, locale)
);


-- =============================================================
-- Reminders

-- The main entry point for reminders (pipeline)
CREATE TABLE IF NOT EXISTS Reminder (
  id INTEGER NOT NULL PRIMARY KEY,
  -- type (t) = 0,
  -- title in localize table. 
  -- question in RemQuestion table. 
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  s_id INTEGER NOT NULL,
  FOREIGN KEY (s_id) REFERENCES Stage (s_id)
);

CREATE TABLE IF NOT EXISTS RemQuestion (
  id INTEGER NOT NULL PRIMARY KEY,
  -- Question in localize table.
  t INTEGER NOT NULL, -- Question type (textbox, select, input, checkbox, etc.)
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  r_id INTEGER NOT NULL,
  FOREIGN KEY (r_id) REFERENCES Reminder (id)
);

