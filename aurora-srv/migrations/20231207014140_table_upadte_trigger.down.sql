-- Add down migration script here
DROP FUNCTION IF EXISTS sqlx_manage_update_time(_tbl regclass);

drop function public.sqlx_set_update_time() cascade;