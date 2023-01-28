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
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (eid);


--
-- PostgreSQL database dump complete
--

