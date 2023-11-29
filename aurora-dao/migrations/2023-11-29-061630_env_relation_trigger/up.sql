-- Your SQL goes here 
-- on table t_ds_environment_relation and t_ds_environment_relation_worker_group insert or update or delete
-- execute procedure notify_env_view_change();
--
-- Your SQL goes here
--
-- Name: t_ds_environment_relation; Type: TABLE; Schema: public; Owner: root
--
CREATE TABLE public.t_ds_environment_relation (
    id integer NOT NULL,
    code bigint NOT NULL,
    name character varying(255) DEFAULT NULL :: character varying,
    config text,
    description text,
    operator integer,
    worker_groups text [],
    create_time timestamp without time zone,
    update_time timestamp without time zone
);

ALTER TABLE
    public.t_ds_environment_relation OWNER TO root;
-- 
CREATE SEQUENCE public.t_ds_environment_relation_id_seq AS integer START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;
-- 
ALTER TABLE
    public.t_ds_environment_relation_id_seq OWNER TO root;
-- Your SQL goes here

ALTER SEQUENCE public.t_ds_environment_relation_id_seq OWNED BY public.t_ds_environment_relation.id;

ALTER TABLE
    public.t_ds_environment_relation_id_seq OWNER TO root;
--
-- Name: t_ds_environment_relation t_ds_environment_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment_relation
ADD
    CONSTRAINT t_ds_environment_relation_pkey PRIMARY KEY (id);
---
ALTER TABLE
    ONLY public.t_ds_environment_relation
ALTER COLUMN
    id
SET
    DEFAULT nextval(
        'public.t_ds_environment_relation_id_seq' :: regclass
    );

SELECT
    pg_catalog.setval(
        'public.t_ds_environment_relation_id_seq',
        1,
        false
    );




--
-- Name: t_ds_environment environment_code_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment_relation
ADD
    CONSTRAINT environment_relation_code_unique UNIQUE (code);

--
-- Name: t_ds_environment_relation environment_relation_name_unique; Type: CONSTRAINT; Schema: public; Owner: root
--
ALTER TABLE
    ONLY public.t_ds_environment_relation
ADD
    CONSTRAINT environment_relation_name_unique UNIQUE (name);


