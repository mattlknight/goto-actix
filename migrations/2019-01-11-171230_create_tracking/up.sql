-- Your SQL goes here
-- Table: public.tracking

-- DROP SEQUENCE public.tracking_id_seq;

CREATE SEQUENCE public.tracking_id_seq;

ALTER SEQUENCE public.tracking_id_seq
    OWNER TO goto;

-- DROP TABLE public.tracking;

CREATE TABLE public.tracking
(
    id integer NOT NULL DEFAULT nextval('tracking_id_seq'::regclass),
    keyword_id integer NOT NULL,
    accessed_on timestamp without time zone NOT NULL,
    CONSTRAINT tracking_pkey PRIMARY KEY (id),
    CONSTRAINT fkey_tracking_keyword FOREIGN KEY (keyword_id)
        REFERENCES public.keywords(id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.tracking
    OWNER to goto;

-- Index: fki_fkey_tracking_keyword

-- DROP INDEX public.fki_fkey_tracking_keyword;

CREATE INDEX fki_fkey_tracking_keyword
    ON public.tracking USING btree
    (keyword_id)
    TABLESPACE pg_default;

-- Index: idx_tracking_accessed

-- DROP INDEX public.idx_tracking_accessed;

CREATE INDEX idx_tracking_accessed
    ON public.tracking USING btree
    (accessed_on)
    TABLESPACE pg_default;

-- Index: idx_tracking_keyword
