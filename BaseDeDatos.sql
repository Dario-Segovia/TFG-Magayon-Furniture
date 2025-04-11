--
-- PostgreSQL database dump
--

-- Dumped from database version 17.4
-- Dumped by pg_dump version 17.2

-- Started on 2025-04-11 14:43:14

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

--
-- TOC entry 2 (class 3079 OID 24805)
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- TOC entry 5043 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- TOC entry 238 (class 1255 OID 24701)
-- Name: encrypt_password(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.encrypt_password() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.password := crypt(NEW.password, gen_salt('bf'));
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.encrypt_password() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 225 (class 1259 OID 24735)
-- Name: clientes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.clientes (
    id integer NOT NULL,
    nombre character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    telefono character varying(20),
    direccion text
);


ALTER TABLE public.clientes OWNER TO postgres;

--
-- TOC entry 224 (class 1259 OID 24734)
-- Name: clientes_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.clientes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.clientes_id_seq OWNER TO postgres;

--
-- TOC entry 5044 (class 0 OID 0)
-- Dependencies: 224
-- Name: clientes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.clientes_id_seq OWNED BY public.clientes.id;


--
-- TOC entry 233 (class 1259 OID 24788)
-- Name: conversiones; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.conversiones (
    id integer NOT NULL,
    tipo_conversion character varying(50) NOT NULL,
    factor numeric(10,5) NOT NULL
);


ALTER TABLE public.conversiones OWNER TO postgres;

--
-- TOC entry 232 (class 1259 OID 24787)
-- Name: conversiones_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.conversiones_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.conversiones_id_seq OWNER TO postgres;

--
-- TOC entry 5045 (class 0 OID 0)
-- Dependencies: 232
-- Name: conversiones_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.conversiones_id_seq OWNED BY public.conversiones.id;


--
-- TOC entry 221 (class 1259 OID 24704)
-- Name: empleados; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.empleados (
    id integer NOT NULL,
    nombre character varying(100) NOT NULL,
    apellido character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    telefono character varying(20),
    puesto character varying(50) NOT NULL,
    salario double precision,
    fecha_contratacion date NOT NULL
);


ALTER TABLE public.empleados OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 24703)
-- Name: empleados_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.empleados_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.empleados_id_seq OWNER TO postgres;

--
-- TOC entry 5046 (class 0 OID 0)
-- Dependencies: 220
-- Name: empleados_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.empleados_id_seq OWNED BY public.empleados.id;


--
-- TOC entry 235 (class 1259 OID 24795)
-- Name: estadisticas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.estadisticas (
    id integer NOT NULL,
    fecha date DEFAULT CURRENT_DATE NOT NULL,
    descripcion text,
    valor numeric(10,2)
);


ALTER TABLE public.estadisticas OWNER TO postgres;

--
-- TOC entry 234 (class 1259 OID 24794)
-- Name: estadisticas_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.estadisticas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.estadisticas_id_seq OWNER TO postgres;

--
-- TOC entry 5047 (class 0 OID 0)
-- Dependencies: 234
-- Name: estadisticas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.estadisticas_id_seq OWNED BY public.estadisticas.id;


--
-- TOC entry 237 (class 1259 OID 24848)
-- Name: horarios; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.horarios (
    id integer NOT NULL,
    empleado_id integer NOT NULL,
    fecha date NOT NULL,
    hora_inicio time without time zone NOT NULL,
    hora_fin time without time zone NOT NULL,
    tipo_turno character varying(50),
    notas text
);


ALTER TABLE public.horarios OWNER TO postgres;

--
-- TOC entry 236 (class 1259 OID 24847)
-- Name: horarios_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.horarios_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.horarios_id_seq OWNER TO postgres;

--
-- TOC entry 5048 (class 0 OID 0)
-- Dependencies: 236
-- Name: horarios_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.horarios_id_seq OWNED BY public.horarios.id;


--
-- TOC entry 223 (class 1259 OID 24725)
-- Name: inventario; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.inventario (
    id integer NOT NULL,
    nombre character varying(100) NOT NULL,
    descripcion text,
    cantidad integer NOT NULL,
    precio numeric(10,2) NOT NULL,
    CONSTRAINT inventario_cantidad_check CHECK ((cantidad >= 0))
);


ALTER TABLE public.inventario OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 24724)
-- Name: inventario_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.inventario_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.inventario_id_seq OWNER TO postgres;

--
-- TOC entry 5049 (class 0 OID 0)
-- Dependencies: 222
-- Name: inventario_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.inventario_id_seq OWNED BY public.inventario.id;


--
-- TOC entry 229 (class 1259 OID 24759)
-- Name: orden_detalle; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.orden_detalle (
    id integer NOT NULL,
    orden_id integer,
    producto_id integer,
    cantidad integer NOT NULL,
    precio_unitario numeric(10,2) NOT NULL,
    CONSTRAINT orden_detalle_cantidad_check CHECK ((cantidad > 0))
);


ALTER TABLE public.orden_detalle OWNER TO postgres;

--
-- TOC entry 228 (class 1259 OID 24758)
-- Name: orden_detalle_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.orden_detalle_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.orden_detalle_id_seq OWNER TO postgres;

--
-- TOC entry 5050 (class 0 OID 0)
-- Dependencies: 228
-- Name: orden_detalle_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.orden_detalle_id_seq OWNED BY public.orden_detalle.id;


--
-- TOC entry 227 (class 1259 OID 24746)
-- Name: ordenes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.ordenes (
    id integer NOT NULL,
    cliente_id integer,
    fecha date DEFAULT CURRENT_DATE NOT NULL,
    total numeric(10,2) NOT NULL
);


ALTER TABLE public.ordenes OWNER TO postgres;

--
-- TOC entry 226 (class 1259 OID 24745)
-- Name: ordenes_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.ordenes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ordenes_id_seq OWNER TO postgres;

--
-- TOC entry 5051 (class 0 OID 0)
-- Dependencies: 226
-- Name: ordenes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.ordenes_id_seq OWNED BY public.ordenes.id;


--
-- TOC entry 231 (class 1259 OID 24777)
-- Name: proveedores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.proveedores (
    id integer NOT NULL,
    nombre character varying(100) NOT NULL,
    contacto character varying(100),
    telefono character varying(20),
    email character varying(100),
    direccion text
);


ALTER TABLE public.proveedores OWNER TO postgres;

--
-- TOC entry 230 (class 1259 OID 24776)
-- Name: proveedores_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.proveedores_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.proveedores_id_seq OWNER TO postgres;

--
-- TOC entry 5052 (class 0 OID 0)
-- Dependencies: 230
-- Name: proveedores_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.proveedores_id_seq OWNED BY public.proveedores.id;


--
-- TOC entry 219 (class 1259 OID 24691)
-- Name: usuarios; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.usuarios (
    id integer NOT NULL,
    usuario character varying(50) NOT NULL,
    password text NOT NULL,
    rol character varying(20) DEFAULT 'empleado'::character varying NOT NULL
);


ALTER TABLE public.usuarios OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 24690)
-- Name: usuarios_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.usuarios_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.usuarios_id_seq OWNER TO postgres;

--
-- TOC entry 5053 (class 0 OID 0)
-- Dependencies: 218
-- Name: usuarios_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.usuarios_id_seq OWNED BY public.usuarios.id;


--
-- TOC entry 4829 (class 2604 OID 24738)
-- Name: clientes id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes ALTER COLUMN id SET DEFAULT nextval('public.clientes_id_seq'::regclass);


--
-- TOC entry 4834 (class 2604 OID 24791)
-- Name: conversiones id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversiones ALTER COLUMN id SET DEFAULT nextval('public.conversiones_id_seq'::regclass);


--
-- TOC entry 4827 (class 2604 OID 24707)
-- Name: empleados id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.empleados ALTER COLUMN id SET DEFAULT nextval('public.empleados_id_seq'::regclass);


--
-- TOC entry 4835 (class 2604 OID 24798)
-- Name: estadisticas id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.estadisticas ALTER COLUMN id SET DEFAULT nextval('public.estadisticas_id_seq'::regclass);


--
-- TOC entry 4837 (class 2604 OID 24851)
-- Name: horarios id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.horarios ALTER COLUMN id SET DEFAULT nextval('public.horarios_id_seq'::regclass);


--
-- TOC entry 4828 (class 2604 OID 24728)
-- Name: inventario id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inventario ALTER COLUMN id SET DEFAULT nextval('public.inventario_id_seq'::regclass);


--
-- TOC entry 4832 (class 2604 OID 24762)
-- Name: orden_detalle id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orden_detalle ALTER COLUMN id SET DEFAULT nextval('public.orden_detalle_id_seq'::regclass);


--
-- TOC entry 4830 (class 2604 OID 24749)
-- Name: ordenes id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ordenes ALTER COLUMN id SET DEFAULT nextval('public.ordenes_id_seq'::regclass);


--
-- TOC entry 4833 (class 2604 OID 24780)
-- Name: proveedores id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedores ALTER COLUMN id SET DEFAULT nextval('public.proveedores_id_seq'::regclass);


--
-- TOC entry 4825 (class 2604 OID 24694)
-- Name: usuarios id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios ALTER COLUMN id SET DEFAULT nextval('public.usuarios_id_seq'::regclass);


--
-- TOC entry 5025 (class 0 OID 24735)
-- Dependencies: 225
-- Data for Name: clientes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.clientes (id, nombre, email, telefono, direccion) FROM stdin;
\.


--
-- TOC entry 5033 (class 0 OID 24788)
-- Dependencies: 233
-- Data for Name: conversiones; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.conversiones (id, tipo_conversion, factor) FROM stdin;
\.


--
-- TOC entry 5021 (class 0 OID 24704)
-- Dependencies: 221
-- Data for Name: empleados; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.empleados (id, nombre, apellido, email, telefono, puesto, salario, fecha_contratacion) FROM stdin;
41	antonomasio	Segovia	asasas@gmail.com	132451345	asdas	134123	2025-04-08
51	Gilberto	Segovia	asdasdas@gmail.com	132451345	asaas	1324123	2025-04-08
43	hola22	si que tal	camion@gmail.com	12341234	julio	23452	2025-04-08
47	Gilbertooide	Segovia	asdzxdssd@gmail.com	132451345	Manubrio	23452	2025-04-08
20	antonomasio	Segovia	as@gmail.com	132451345	Chimpance	546456	2025-04-08
\.


--
-- TOC entry 5035 (class 0 OID 24795)
-- Dependencies: 235
-- Data for Name: estadisticas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.estadisticas (id, fecha, descripcion, valor) FROM stdin;
\.


--
-- TOC entry 5037 (class 0 OID 24848)
-- Dependencies: 237
-- Data for Name: horarios; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.horarios (id, empleado_id, fecha, hora_inicio, hora_fin, tipo_turno, notas) FROM stdin;
14	20	2025-04-18	08:00:00	17:00:00	Turno noche	kk
17	41	2025-04-09	08:00:00	17:00:00	Turno noche	ass
18	41	2025-04-11	08:00:00	17:00:00	Turno extra	si que tal
\.


--
-- TOC entry 5023 (class 0 OID 24725)
-- Dependencies: 223
-- Data for Name: inventario; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.inventario (id, nombre, descripcion, cantidad, precio) FROM stdin;
\.


--
-- TOC entry 5029 (class 0 OID 24759)
-- Dependencies: 229
-- Data for Name: orden_detalle; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.orden_detalle (id, orden_id, producto_id, cantidad, precio_unitario) FROM stdin;
\.


--
-- TOC entry 5027 (class 0 OID 24746)
-- Dependencies: 227
-- Data for Name: ordenes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.ordenes (id, cliente_id, fecha, total) FROM stdin;
\.


--
-- TOC entry 5031 (class 0 OID 24777)
-- Dependencies: 231
-- Data for Name: proveedores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.proveedores (id, nombre, contacto, telefono, email, direccion) FROM stdin;
\.


--
-- TOC entry 5019 (class 0 OID 24691)
-- Dependencies: 219
-- Data for Name: usuarios; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.usuarios (id, usuario, password, rol) FROM stdin;
1	Dario	$2a$06$ASYGxkAW6VRMK07ezfmLLOGvcc0aL4SMnNg.TAlR4yuzjsNaeJ1bS	admin
2	prueba	$2a$06$YwllLFH7o9T9UNDkeHBc5O9V0142TIO4Jr4mijL8edDDIvjSvHH.O	admin
3	empleado	$2a$06$u5o2ENj3xH8CTZsFjbeOUOBX4sEB98nF0SqDxaJfKS/08CtWaAjQG	empleado
\.


--
-- TOC entry 5054 (class 0 OID 0)
-- Dependencies: 224
-- Name: clientes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.clientes_id_seq', 1, false);


--
-- TOC entry 5055 (class 0 OID 0)
-- Dependencies: 232
-- Name: conversiones_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.conversiones_id_seq', 1, false);


--
-- TOC entry 5056 (class 0 OID 0)
-- Dependencies: 220
-- Name: empleados_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.empleados_id_seq', 53, true);


--
-- TOC entry 5057 (class 0 OID 0)
-- Dependencies: 234
-- Name: estadisticas_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.estadisticas_id_seq', 1, false);


--
-- TOC entry 5058 (class 0 OID 0)
-- Dependencies: 236
-- Name: horarios_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.horarios_id_seq', 18, true);


--
-- TOC entry 5059 (class 0 OID 0)
-- Dependencies: 222
-- Name: inventario_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.inventario_id_seq', 1, false);


--
-- TOC entry 5060 (class 0 OID 0)
-- Dependencies: 228
-- Name: orden_detalle_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.orden_detalle_id_seq', 1, false);


--
-- TOC entry 5061 (class 0 OID 0)
-- Dependencies: 226
-- Name: ordenes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.ordenes_id_seq', 1, false);


--
-- TOC entry 5062 (class 0 OID 0)
-- Dependencies: 230
-- Name: proveedores_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.proveedores_id_seq', 1, false);


--
-- TOC entry 5063 (class 0 OID 0)
-- Dependencies: 218
-- Name: usuarios_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.usuarios_id_seq', 1, false);


--
-- TOC entry 4851 (class 2606 OID 24744)
-- Name: clientes clientes_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT clientes_email_key UNIQUE (email);


--
-- TOC entry 4853 (class 2606 OID 24742)
-- Name: clientes clientes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT clientes_pkey PRIMARY KEY (id);


--
-- TOC entry 4863 (class 2606 OID 24793)
-- Name: conversiones conversiones_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversiones
    ADD CONSTRAINT conversiones_pkey PRIMARY KEY (id);


--
-- TOC entry 4845 (class 2606 OID 24711)
-- Name: empleados empleados_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.empleados
    ADD CONSTRAINT empleados_email_key UNIQUE (email);


--
-- TOC entry 4847 (class 2606 OID 24709)
-- Name: empleados empleados_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.empleados
    ADD CONSTRAINT empleados_pkey PRIMARY KEY (id);


--
-- TOC entry 4865 (class 2606 OID 24803)
-- Name: estadisticas estadisticas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.estadisticas
    ADD CONSTRAINT estadisticas_pkey PRIMARY KEY (id);


--
-- TOC entry 4867 (class 2606 OID 24855)
-- Name: horarios horarios_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.horarios
    ADD CONSTRAINT horarios_pkey PRIMARY KEY (id);


--
-- TOC entry 4849 (class 2606 OID 24733)
-- Name: inventario inventario_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inventario
    ADD CONSTRAINT inventario_pkey PRIMARY KEY (id);


--
-- TOC entry 4857 (class 2606 OID 24765)
-- Name: orden_detalle orden_detalle_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orden_detalle
    ADD CONSTRAINT orden_detalle_pkey PRIMARY KEY (id);


--
-- TOC entry 4855 (class 2606 OID 24752)
-- Name: ordenes ordenes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ordenes
    ADD CONSTRAINT ordenes_pkey PRIMARY KEY (id);


--
-- TOC entry 4859 (class 2606 OID 24786)
-- Name: proveedores proveedores_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedores
    ADD CONSTRAINT proveedores_email_key UNIQUE (email);


--
-- TOC entry 4861 (class 2606 OID 24784)
-- Name: proveedores proveedores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedores
    ADD CONSTRAINT proveedores_pkey PRIMARY KEY (id);


--
-- TOC entry 4841 (class 2606 OID 24698)
-- Name: usuarios usuarios_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT usuarios_pkey PRIMARY KEY (id);


--
-- TOC entry 4843 (class 2606 OID 24700)
-- Name: usuarios usuarios_usuario_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT usuarios_usuario_key UNIQUE (usuario);


--
-- TOC entry 4872 (class 2620 OID 24702)
-- Name: usuarios encrypt_user_password; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER encrypt_user_password BEFORE INSERT OR UPDATE ON public.usuarios FOR EACH ROW EXECUTE FUNCTION public.encrypt_password();


--
-- TOC entry 4871 (class 2606 OID 24856)
-- Name: horarios fk_empleado; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.horarios
    ADD CONSTRAINT fk_empleado FOREIGN KEY (empleado_id) REFERENCES public.empleados(id) ON DELETE CASCADE;


--
-- TOC entry 4869 (class 2606 OID 24766)
-- Name: orden_detalle orden_detalle_orden_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orden_detalle
    ADD CONSTRAINT orden_detalle_orden_id_fkey FOREIGN KEY (orden_id) REFERENCES public.ordenes(id) ON DELETE CASCADE;


--
-- TOC entry 4870 (class 2606 OID 24771)
-- Name: orden_detalle orden_detalle_producto_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orden_detalle
    ADD CONSTRAINT orden_detalle_producto_id_fkey FOREIGN KEY (producto_id) REFERENCES public.inventario(id) ON DELETE SET NULL;


--
-- TOC entry 4868 (class 2606 OID 24753)
-- Name: ordenes ordenes_cliente_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ordenes
    ADD CONSTRAINT ordenes_cliente_id_fkey FOREIGN KEY (cliente_id) REFERENCES public.clientes(id) ON DELETE SET NULL;


-- Completed on 2025-04-11 14:43:15

--
-- PostgreSQL database dump complete
--

