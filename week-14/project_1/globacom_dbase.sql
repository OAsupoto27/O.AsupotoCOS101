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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(10) NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration character varying(15) NOT NULL,
    p_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, p_managerid) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


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
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (p_managerid);


--
-- Name: staff staff_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT staff_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

