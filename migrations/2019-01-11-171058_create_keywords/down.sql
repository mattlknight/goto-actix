-- This file should undo anything in `up.sql`
DROP INDEX public.idx_keywords_modified;
DROP INDEX public.idx_keywords_keyword;
DROP INDEX public.idx_keywords_created;
DROP TABLE public.keywords;
DROP SEQUENCE public.keywords_id_seq;
