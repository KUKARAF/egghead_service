-- Add support for file attachments and git versioning

ALTER TABLE tasks ADD COLUMN files_json TEXT;
-- JSON array of {name, content} pairs, NULL if no files attached

ALTER TABLE tasks ADD COLUMN git_sha TEXT;
-- Commit SHA from git repo when script is written, NULL until generation completes
