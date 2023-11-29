-- Your SQL goes here
create view v_ds_environment (id, code, name, config, description, operator, create_time, update_time, worker_groups) as
SELECT t.id,
       t.code,
       t.name,
       t.config,
       t.description,
       t.operator,
       t.create_time,
       t.update_time,
       array_agg(g.worker_group) AS worker_groups
FROM t_ds_environment t
         LEFT JOIN t_ds_environment_worker_group_relation g ON t.code = g.environment_code
GROUP BY t.id, t.code, t.config, t.name, t.create_time, t.description, t.operator, t.update_time;

alter table v_ds_environment
    owner to root;

