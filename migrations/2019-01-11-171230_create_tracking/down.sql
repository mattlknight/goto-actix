-- This file should undo anything in `up.sql`
DROP INDEX public.idx_tracking_accessed;
DROP INDEX public.fki_fkey_tracking_keyword;
DROP TABLE public.tracking;
DROP SEQUENCE public.tracking_id_seq;
