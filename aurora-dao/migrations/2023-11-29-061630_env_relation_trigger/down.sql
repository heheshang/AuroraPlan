-- This file should undo anything in `up.sql`
ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment_relation DROP CONSTRAINT IF EXISTS t_ds_environment_relation_pkey;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment_relation DROP CONSTRAINT IF EXISTS environment_relation_name_unique;

ALTER TABLE
    IF EXISTS ONLY public.t_ds_environment_relation DROP CONSTRAINT IF EXISTS environment_relation_code_unique;

DROP TABLE IF EXISTS public.t_ds_environment_relation;
DROP SEQUENCE IF EXISTS public.t_ds_environment_relation_id_seq;