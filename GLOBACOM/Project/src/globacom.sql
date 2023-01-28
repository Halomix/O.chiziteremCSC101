--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
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
    c_name character varying(255),
    c_age integer,
    c_email character varying(255),
    c_mobile character varying(15),
    eid integer,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(255),
    data_duration integer,
    data_price integer
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: dataplan_data_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.dataplan_data_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.dataplan_data_id_seq OWNER TO postgres;

--
-- Name: dataplan_data_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.dataplan_data_id_seq OWNED BY public.dataplan.data_id;


--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dno integer NOT NULL,
    dept_managerid integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pname character(1) NOT NULL,
    pno integer NOT NULL,
    project_managerid integer NOT NULL,
    pduration character varying(50) NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    eid integer NOT NULL,
    ename text NOT NULL,
    dno integer NOT NULL,
    esal real NOT NULL,
    age integer NOT NULL,
    phone character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Name: dataplan data_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan ALTER COLUMN data_id SET DEFAULT nextval('public.dataplan_data_id_seq'::regclass);


--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	07056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
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
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dno, dept_managerid, dname, dlocation, pno) FROM stdin;
1	108	Administration	Ikeja	44
2	101	Account	Egbeda	11
3	100	Packaging	Ajah	44
4	120	Research	V.I	33
5	97	Account	Magodo	22
6	122	Operations	Mile 2	44
7	107	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pname, pno, project_managerid, pduration) FROM stdin;
A	11	102	9 Months
B	22	97	14 Months
C	33	120	16 Months
D	44	108	25 Months
E	55	107	9 Months
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (eid, ename, dno, esal, age, phone) FROM stdin;
101	ALADE JOY	2	250000	33	8023089832
100	MUSTAPHA ALI	3	175000	32	8063285831
107	ALOKWE MARTIN	7	380000	48	7090082812
97	DANKADE AMINAT	5	550000	40	9023688832
108	JOSIAH JOSHUA	1	120000	30	8053189131
102	MAKINDE MARY	2	450000	55	9023487830
120	ADELEKE JANE	4	200000	38	7061045862
122	OSAHON MARK	6	320000	44	8022289842
117	SULEMAN AJAYI	3	800000	50	7030089811
104	KUTI LAWAL	1	750000	35	9145689842
\.


--
-- Name: dataplan_data_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.dataplan_data_id_seq', 11, true);


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
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dno);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (eid);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pname);


--
-- PostgreSQL database dump complete
--

