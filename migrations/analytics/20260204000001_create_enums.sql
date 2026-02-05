-- data_source enum: where the URL was resolved from
DO $$ BEGIN
    CREATE TYPE data_source AS ENUM ('cache', 'database');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;
