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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(25) NOT NULL,
    c_mobile character varying(15) NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055891112	100	5
111	Lilian Jaiye	43	l_jaiye@gmail.com	08055185341	101	3
112	Arthur Musa	25	a_musa@gmail.com	08054123456	140	2
113	Philip Akonjo	34	p_akonjo@gmail.com	09052356772	100	10
114	Marylene Mapa	40	m_mapa@gmail.com	07055336677	120	15
115	Ogheneso Agor	31	o_ogor@gmail.com	07055567744	110	1
116	Adams Bree	36	a_bree@gmail.com	08056677432	102	11
117	Gafson Mathias	42	g_mathias@gmail.com	08056764324	124	6
118	Adesola Adeleke	29	a_adeleke@gmail.com	08056772143	107	9
119	Samuel Tamire	45	s_tamire@gmail.com	07055123444	117	13
120	James Job	37	j_job@gmail.com	08056714321	103	7
121	Matthew Jakande	39	m_jakande@gmail.com	08056211234	100	8
122	Jamiu Adegboye	50	j_adegboye@gmail.com	08054921923	107	12
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

