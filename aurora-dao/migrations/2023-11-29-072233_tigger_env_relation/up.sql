-- Your SQL goes here
-- trigger for add/update/delete a t_ds_environment
CREATE OR REPLACE FUNCTION t_ds_environment_trigger() RETURNS trigger AS $$ 

BEGIN 
    IF (TG_OP = 'INSERT') THEN
        insert into
            t_ds_environment_relation
        SELECT
            t.id,
            t.code,
            t.name,
            t.config,
            t.description,
            t.operator,
            array_agg(g.worker_group) AS worker_groups,
            t.create_time,
            t.update_time
        FROM
            t_ds_environment t
            LEFT JOIN t_ds_environment_worker_group_relation g ON t.code = g.environment_code
        where
            t.code not in (
                select
                    code
                from
                    t_ds_environment_relation
            ) 
        GROUP BY
            t.id,
            t.code,
            t.config,
            t.name,
            t.create_time,
            t.description,
            t.operator,
            t.update_time;

-- update t_ds_environment_relation table
-- check if the t_ds_environment is valid
-- check if the t_ds_environment is conflicting with other t_ds_environments
-- if not, insert the t_ds_environment
-- if yes, return error
ELSIF (TG_OP = 'UPDATE') THEN -- if status is changed, update t_ds_environment_relation table
    IF (OLD.update_time != NEW.update_time) THEN
        delete from
            t_ds_environment_relation
        where
            code = OLD.code;

        insert into
            t_ds_environment_relation
        SELECT
            t.id,
            t.code,
            t.name,
            t.config,
            t.description,
            t.operator,
            array_agg(g.worker_group) AS worker_groups,
            t.create_time,
            t.update_time
        FROM
            t_ds_environment t
            LEFT JOIN t_ds_environment_worker_group_relation g ON t.code = g.environment_code
        where
            t.code = OLD.code
        GROUP BY
            t.id,
            t.code,
            t.config,
            t.name,
            t.create_time,
            t.description,
            t.operator,
            t.update_time;

    END IF;

-- check if the t_ds_environment is valid
-- check if the t_ds_environment is conflicting with other t_ds_environments
-- if not, update the t_ds_environment
-- if yes, return error
ELSIF (TG_OP = 'DELETE') THEN -- update t_ds_environment_relation table
    delete from
        t_ds_environment_relation
    where
        code = OLD.code;

-- delete the t_ds_environment
END IF;

-- notify a channel called t_ds_environment_change
NOTIFY t_ds_environment_update;

RETURN NULL;

END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER environment_trigger
AFTER
INSERT
    OR
UPDATE
    OR DELETE ON t_ds_environment FOR EACH ROW EXECUTE PROCEDURE t_ds_environment_trigger();