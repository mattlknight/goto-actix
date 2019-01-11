-- Your SQL goes here
-- Table: public.keywords

-- DROP SEQUENCE public.keywords_id_seq;

CREATE SEQUENCE public.keywords_id_seq;

ALTER SEQUENCE public.keywords_id_seq
    OWNER TO goto;

-- DROP TABLE public.keywords;

CREATE TABLE public.keywords
(
    id integer NOT NULL DEFAULT nextval('keywords_id_seq'::regclass),
    keyword text COLLATE pg_catalog."default" NOT NULL,
    url text COLLATE pg_catalog."default" NOT NULL,
    created_on timestamp without time zone NOT NULL default now(),
    modified_on timestamp without time zone NOT NULL default now(),
    CONSTRAINT keywords_pkey PRIMARY KEY (id)
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.keywords
    OWNER to goto;

-- Index: idx_keywords_created

-- DROP INDEX public.idx_keywords_created;

CREATE INDEX idx_keywords_created
    ON public.keywords USING btree
    (created_on)
    TABLESPACE pg_default;

-- Index: idx_keywords_keyword

-- DROP INDEX public.idx_keywords_keyword;

CREATE UNIQUE INDEX idx_keywords_keyword
    ON public.keywords USING btree
    (keyword COLLATE pg_catalog."default")
    TABLESPACE pg_default;

-- Index: idx_keywords_modified

-- DROP INDEX public.idx_keywords_modified;

CREATE INDEX idx_keywords_modified
    ON public.keywords USING btree
    (modified_on)
    TABLESPACE pg_default;
