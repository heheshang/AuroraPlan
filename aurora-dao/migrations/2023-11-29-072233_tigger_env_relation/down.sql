-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS environment_trigger on t_ds_environment ;
DROP FUNCTION IF EXISTS t_ds_environment_trigger;