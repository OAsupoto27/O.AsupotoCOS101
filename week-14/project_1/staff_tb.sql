--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    staff_sal integer NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	ALADE JOY	2	250000	33	8023089832
108	MUSTAPHA ALI	3	175000	32	8063285831
107	ALOKWE MARTIN	7	380000	48	7900828212
97	DANKADE AMINAT	1	550000	40	9023688832
103	JOSIAH JOSHUA	1	120000	30	8053189131
105	MANKINDE MARY	3	450000	35	9023487830
104	ADELEKE JANE	1	250000	40	7061045862
140	OSAHON MARK	1	320000	45	8022289842
117	KUTI LAWAL	3	750000	34	9145689842
109	SULEMAN AJAYI	3	800000	50	7030089981
\.


--
-- Name: staff staff_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT staff_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

