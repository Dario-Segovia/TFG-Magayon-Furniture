--
-- PostgreSQL database dump
--

-- Dumped from database version 17.4
-- Dumped by pg_dump version 17.4

-- Started on 2025-05-19 12:11:04

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
-- TOC entry 5081 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- TOC entry 282 (class 1255 OID 25956)
-- Name: actualizar_stock_compra(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.actualizar_stock_compra() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE inventario
    SET cantidad = cantidad + NEW.cantidad
    WHERE id = NEW.id_producto;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.actualizar_stock_compra() OWNER TO postgres;

--
-- TOC entry 281 (class 1255 OID 25954)
-- Name: actualizar_stock_venta(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.actualizar_stock_venta() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    UPDATE inventario
    SET cantidad = cantidad - NEW.cantidad
    WHERE id = NEW.id_producto;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.actualizar_stock_venta() OWNER TO postgres;

--
-- TOC entry 244 (class 1255 OID 24701)
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
-- TOC entry 223 (class 1259 OID 24735)
-- Name: clientes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.clientes (
    id integer NOT NULL,
    nombre character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    telefono character varying(20),
    via character varying(100),
    numero character varying(10),
    ciudad character varying(50),
    provincia character varying(50),
    pais character varying(50)
);


ALTER TABLE public.clientes OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 24734)
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
-- TOC entry 5082 (class 0 OID 0)
-- Dependencies: 222
-- Name: clientes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.clientes_id_seq OWNED BY public.clientes.id;


--
-- TOC entry 239 (class 1259 OID 25930)
-- Name: compras; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.compras (
    id integer NOT NULL,
    fecha timestamp without time zone DEFAULT now() NOT NULL,
    id_proveedor integer,
    total numeric(10,2) NOT NULL
);


ALTER TABLE public.compras OWNER TO postgres;

--
-- TOC entry 238 (class 1259 OID 25929)
-- Name: compras_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.compras_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.compras_id_seq OWNER TO postgres;

--
-- TOC entry 5083 (class 0 OID 0)
-- Dependencies: 238
-- Name: compras_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.compras_id_seq OWNED BY public.compras.id;


--
-- TOC entry 227 (class 1259 OID 24788)
-- Name: conversiones; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.conversiones (
    id integer NOT NULL,
    tipo_conversion character varying(50) NOT NULL,
    factor numeric(10,5) NOT NULL
);


ALTER TABLE public.conversiones OWNER TO postgres;

--
-- TOC entry 226 (class 1259 OID 24787)
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
-- TOC entry 5084 (class 0 OID 0)
-- Dependencies: 226
-- Name: conversiones_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.conversiones_id_seq OWNED BY public.conversiones.id;


--
-- TOC entry 241 (class 1259 OID 25938)
-- Name: detalle_compras; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.detalle_compras (
    id integer NOT NULL,
    id_compra integer NOT NULL,
    id_producto integer NOT NULL,
    cantidad integer NOT NULL,
    precio_unitario numeric(10,2) NOT NULL
);


ALTER TABLE public.detalle_compras OWNER TO postgres;

--
-- TOC entry 240 (class 1259 OID 25937)
-- Name: detalle_compras_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.detalle_compras_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.detalle_compras_id_seq OWNER TO postgres;

--
-- TOC entry 5085 (class 0 OID 0)
-- Dependencies: 240
-- Name: detalle_compras_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.detalle_compras_id_seq OWNED BY public.detalle_compras.id;


--
-- TOC entry 237 (class 1259 OID 25913)
-- Name: detalle_ventas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.detalle_ventas (
    id integer NOT NULL,
    id_venta integer NOT NULL,
    id_producto integer NOT NULL,
    cantidad integer NOT NULL,
    precio_unitario numeric(10,2) NOT NULL
);


ALTER TABLE public.detalle_ventas OWNER TO postgres;

--
-- TOC entry 236 (class 1259 OID 25912)
-- Name: detalle_ventas_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.detalle_ventas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.detalle_ventas_id_seq OWNER TO postgres;

--
-- TOC entry 5086 (class 0 OID 0)
-- Dependencies: 236
-- Name: detalle_ventas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.detalle_ventas_id_seq OWNED BY public.detalle_ventas.id;


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
-- TOC entry 5087 (class 0 OID 0)
-- Dependencies: 220
-- Name: empleados_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.empleados_id_seq OWNED BY public.empleados.id;


--
-- TOC entry 229 (class 1259 OID 24795)
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
-- TOC entry 228 (class 1259 OID 24794)
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
-- TOC entry 5088 (class 0 OID 0)
-- Dependencies: 228
-- Name: estadisticas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.estadisticas_id_seq OWNED BY public.estadisticas.id;


--
-- TOC entry 231 (class 1259 OID 24848)
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
-- TOC entry 230 (class 1259 OID 24847)
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
-- TOC entry 5089 (class 0 OID 0)
-- Dependencies: 230
-- Name: horarios_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.horarios_id_seq OWNED BY public.horarios.id;


--
-- TOC entry 233 (class 1259 OID 25818)
-- Name: inventario; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.inventario (
    id integer NOT NULL,
    nombre text NOT NULL,
    descripcion text,
    cantidad integer DEFAULT 0 NOT NULL,
    precio_unitario numeric(10,2) NOT NULL,
    categoria text
);


ALTER TABLE public.inventario OWNER TO postgres;

--
-- TOC entry 232 (class 1259 OID 25817)
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
-- TOC entry 5090 (class 0 OID 0)
-- Dependencies: 232
-- Name: inventario_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.inventario_id_seq OWNED BY public.inventario.id;


--
-- TOC entry 243 (class 1259 OID 25968)
-- Name: proveedor_productos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.proveedor_productos (
    id integer NOT NULL,
    proveedor_id integer NOT NULL,
    producto text NOT NULL,
    descripcion text,
    precio_unitario numeric(10,2) NOT NULL,
    categoria text
);


ALTER TABLE public.proveedor_productos OWNER TO postgres;

--
-- TOC entry 242 (class 1259 OID 25967)
-- Name: proveedor_productos_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.proveedor_productos_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.proveedor_productos_id_seq OWNER TO postgres;

--
-- TOC entry 5091 (class 0 OID 0)
-- Dependencies: 242
-- Name: proveedor_productos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.proveedor_productos_id_seq OWNED BY public.proveedor_productos.id;


--
-- TOC entry 225 (class 1259 OID 24777)
-- Name: proveedores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.proveedores (
    id integer NOT NULL,
    nombre character varying(100) NOT NULL,
    contacto character varying(100),
    telefono character varying(20),
    email character varying(100),
    direccion text,
    pais character varying(100),
    estado character varying(20) DEFAULT 'activo'::character varying,
    contrato_vigente boolean DEFAULT false
);


ALTER TABLE public.proveedores OWNER TO postgres;

--
-- TOC entry 224 (class 1259 OID 24776)
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
-- TOC entry 5092 (class 0 OID 0)
-- Dependencies: 224
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
-- TOC entry 5093 (class 0 OID 0)
-- Dependencies: 218
-- Name: usuarios_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.usuarios_id_seq OWNED BY public.usuarios.id;


--
-- TOC entry 235 (class 1259 OID 25905)
-- Name: ventas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.ventas (
    id integer NOT NULL,
    fecha timestamp without time zone DEFAULT now() NOT NULL,
    id_cliente integer,
    total numeric(10,2) NOT NULL
);


ALTER TABLE public.ventas OWNER TO postgres;

--
-- TOC entry 234 (class 1259 OID 25904)
-- Name: ventas_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.ventas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ventas_id_seq OWNER TO postgres;

--
-- TOC entry 5094 (class 0 OID 0)
-- Dependencies: 234
-- Name: ventas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.ventas_id_seq OWNED BY public.ventas.id;


--
-- TOC entry 4845 (class 2604 OID 24738)
-- Name: clientes id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes ALTER COLUMN id SET DEFAULT nextval('public.clientes_id_seq'::regclass);


--
-- TOC entry 4858 (class 2604 OID 25933)
-- Name: compras id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.compras ALTER COLUMN id SET DEFAULT nextval('public.compras_id_seq'::regclass);


--
-- TOC entry 4849 (class 2604 OID 24791)
-- Name: conversiones id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversiones ALTER COLUMN id SET DEFAULT nextval('public.conversiones_id_seq'::regclass);


--
-- TOC entry 4860 (class 2604 OID 25941)
-- Name: detalle_compras id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_compras ALTER COLUMN id SET DEFAULT nextval('public.detalle_compras_id_seq'::regclass);


--
-- TOC entry 4857 (class 2604 OID 25916)
-- Name: detalle_ventas id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_ventas ALTER COLUMN id SET DEFAULT nextval('public.detalle_ventas_id_seq'::regclass);


--
-- TOC entry 4844 (class 2604 OID 24707)
-- Name: empleados id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.empleados ALTER COLUMN id SET DEFAULT nextval('public.empleados_id_seq'::regclass);


--
-- TOC entry 4850 (class 2604 OID 24798)
-- Name: estadisticas id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.estadisticas ALTER COLUMN id SET DEFAULT nextval('public.estadisticas_id_seq'::regclass);


--
-- TOC entry 4852 (class 2604 OID 24851)
-- Name: horarios id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.horarios ALTER COLUMN id SET DEFAULT nextval('public.horarios_id_seq'::regclass);


--
-- TOC entry 4853 (class 2604 OID 25821)
-- Name: inventario id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inventario ALTER COLUMN id SET DEFAULT nextval('public.inventario_id_seq'::regclass);


--
-- TOC entry 4861 (class 2604 OID 25971)
-- Name: proveedor_productos id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedor_productos ALTER COLUMN id SET DEFAULT nextval('public.proveedor_productos_id_seq'::regclass);


--
-- TOC entry 4846 (class 2604 OID 24780)
-- Name: proveedores id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedores ALTER COLUMN id SET DEFAULT nextval('public.proveedores_id_seq'::regclass);


--
-- TOC entry 4842 (class 2604 OID 24694)
-- Name: usuarios id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios ALTER COLUMN id SET DEFAULT nextval('public.usuarios_id_seq'::regclass);


--
-- TOC entry 4855 (class 2604 OID 25908)
-- Name: ventas id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ventas ALTER COLUMN id SET DEFAULT nextval('public.ventas_id_seq'::regclass);


--
-- TOC entry 5055 (class 0 OID 24735)
-- Dependencies: 223
-- Data for Name: clientes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.clientes (id, nombre, email, telefono, via, numero, ciudad, provincia, pais) FROM stdin;
2	wiwiwi	wawawa@gmail.com	12341234	Tika masala	2	New york	New york	United States 
3	Tumtumtum Sahur	tumtum@gmail.com	666888555	Calle flores	1	Madrid	Madrid	España
4	mamammam	mimimi@gmail.com	13412341	Petonia St. Buenomar	45 	Quezon city	Metro Manila	Philippines 
5	Juan Pérez	juan.perez@example.com	600123456	Calle Mayor	12	Madrid	Madrid	España
6	Laura Gómez	laura.gomez@example.com	611654987	Avenida del Sol	45	Sevilla	Sevilla	España
7	Carlos Martín	carlos.martin@example.com	622789456	Paseo de Gracia	10	Barcelona	Barcelona	España
8	Ana Ruiz	ana.ruiz@example.com	633987654	Calle Real	5	Valencia	Valencia	España
9	Pedro López	pedro.lopez@example.com	644112233	Calle Luna	18	Bilbao	Bizkaia	España
10	Marta Sánchez	marta.sanchez@example.com	655334455	Calle del Mar	22	Málaga	Málaga	España
11	Luis Ramírez	luis.ramirez@example.com	666556677	Calle Olmo	7	Granada	Granada	España
12	Sara Vega	sara.vega@example.com	677778899	Calle Jardín	3	Zaragoza	Zaragoza	España
13	Diego Mora	diego.mora@example.com	688990011	Camino Viejo	30	Murcia	Murcia	España
14	Elena Navarro	elena.navarro@example.com	699112244	Plaza Mayor	1	A Coruña	A Coruña	España
1	asesino	bobardirocrocodilo@gmail.com	66666666	Avenida Guatemala	38	Huelva	Huelva	España
\.


--
-- TOC entry 5071 (class 0 OID 25930)
-- Dependencies: 239
-- Data for Name: compras; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.compras (id, fecha, id_proveedor, total) FROM stdin;
41	2025-05-14 17:45:31	1	597.99
46	2025-05-16 13:33:34.440843	1	65.00
47	2025-05-16 13:34:29.759854	3	50.00
\.


--
-- TOC entry 5059 (class 0 OID 24788)
-- Dependencies: 227
-- Data for Name: conversiones; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.conversiones (id, tipo_conversion, factor) FROM stdin;
\.


--
-- TOC entry 5073 (class 0 OID 25938)
-- Dependencies: 241
-- Data for Name: detalle_compras; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.detalle_compras (id, id_compra, id_producto, cantidad, precio_unitario) FROM stdin;
42	41	5	2	49.00
43	41	10	1	499.99
45	46	16	1	65.00
46	47	17	1	50.00
\.


--
-- TOC entry 5069 (class 0 OID 25913)
-- Dependencies: 237
-- Data for Name: detalle_ventas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.detalle_ventas (id, id_venta, id_producto, cantidad, precio_unitario) FROM stdin;
6	4	7	3	199.99
7	5	10	1	210.00
8	5	3	1	159.99
10	2	4	3	899.99
12	6	7	1	199.99
13	6	9	1	320.00
\.


--
-- TOC entry 5053 (class 0 OID 24704)
-- Dependencies: 221
-- Data for Name: empleados; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.empleados (id, nombre, apellido, email, telefono, puesto, salario, fecha_contratacion) FROM stdin;
41	antonomasio	Segovia	asasas@gmail.com	132451345	asdas	134123	2025-04-08
43	hola22	si que tal	camion@gmail.com	12341234	julio	23452	2025-04-08
47	Gilbertooide	Segovia	asdzxdssd@gmail.com	132451345	Manubrio	23452	2025-04-08
51	Gilberto	Segoviaruiz	asdasdas@gmail.com	132451345	asaas	1324123	2025-04-08
20	antonomasio	Segovia	as@gmail.com	132451345	Chimpance	546456	2025-04-08
\.


--
-- TOC entry 5061 (class 0 OID 24795)
-- Dependencies: 229
-- Data for Name: estadisticas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.estadisticas (id, fecha, descripcion, valor) FROM stdin;
\.


--
-- TOC entry 5063 (class 0 OID 24848)
-- Dependencies: 231
-- Data for Name: horarios; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.horarios (id, empleado_id, fecha, hora_inicio, hora_fin, tipo_turno, notas) FROM stdin;
14	20	2025-04-18	08:00:00	17:00:00	Turno noche	kk
17	41	2025-04-09	08:00:00	17:00:00	Turno noche	ass
18	41	2025-04-11	08:00:00	17:00:00	Turno extra	si que tal
19	41	2025-04-10	08:00:00	17:00:00	Turno normal	esto es una prueba
20	43	2025-04-17	08:00:00	17:00:00	Turno extra	patata
21	41	2025-04-12	08:00:00	19:00:00	Turno noche	patatat
24	47	2025-05-22	08:00:00	17:00:00	Turno noche	asasas
\.


--
-- TOC entry 5065 (class 0 OID 25818)
-- Dependencies: 233
-- Data for Name: inventario; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.inventario (id, nombre, descripcion, cantidad, precio_unitario, categoria) FROM stdin;
17	Pruebecita123	pruebecon	1	50.00	birmania
16	Estantería Modular	estanteria	1	65.00	mueblecitos
19	popo	pepe	0	56.00	papa
20	prueba	pizza	0	0.00	sisisis
15	asas	asas	10	100.00	Locura
6	Mesa Comedor	Mesa extensible 6 personas	5	350.00	Comedor
3	Mesa de Centro	Mesa de centro madera roble	11	159.99	Salón
4	Cama King Size	Cama tamaño king con cabecero	-6	899.99	Dormitorio
7	Silla Oficina	Silla ergonómica de oficina	10	199.99	Oficina
9	Cómoda 5 Cajones	Cómoda madera natural	9	320.00	Dormitorio
8	Armario 2 Puertas	Armario ropero blanco	102	450.00	Dormitorio
2	Butaca Relax	Butaca reclinable cuero negro	6	299.99	Salón
1	Sofá 3 Plazas	Sofá de tela gris moderno	284	499.99	Salón
5	Estantería Modular	Estantería blanca modular	161	120.50	Salón
10	Escritorio Compacto	Escritorio pequeño de trabajo	9	210.00	Oficina
\.


--
-- TOC entry 5075 (class 0 OID 25968)
-- Dependencies: 243
-- Data for Name: proveedor_productos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.proveedor_productos (id, proveedor_id, producto, descripcion, precio_unitario, categoria) FROM stdin;
10	1	Sofá 3 Plazas	Sofá de tela gris moderno	499.99	Salón
11	1	Estantería Modular	prueba	65.00	Salón
12	3	Pruebecita123	la muy pruebona	50.00	birmania
13	3	enerin	juevecin	45.00	kali
17	14	popo	pepe	56.00	papa
18	14	prueba	pizza	0.00	sisisis
\.


--
-- TOC entry 5057 (class 0 OID 24777)
-- Dependencies: 225
-- Data for Name: proveedores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.proveedores (id, nombre, contacto, telefono, email, direccion, pais, estado, contrato_vigente) FROM stdin;
28	Fábrica de Sueños	Ricardo Gómez	655889900	pedidos@fabricadesuenos.com	Calle ejemplo	España	activo	f
29	Espacio y Estilo	Laura Sánchez	644990011	contacto@espacioyestilo.com	Calle ejemplo	España	activo	f
30	MegaMuebles	Antonio Pérez	633221100	ventas@megamuebles.com	Calle ejemplo	España	activo	f
31	Sillas y Más	Julia Ortega	622334455	ventas@sillasyplus.com	Calle ejemplo	España	activo	f
11	tumtum	sahur	666888999	tumtum@gmail.com	Calle flores 1	España	inactivo	t
12	Suministros Industriales del Sur SA	Ana Pérez	959123456	ana.perez@suministrosdelsur.es	Polígono Industrial La Paz, Calle B, Nave 5	España	activo	t
13	Maquinaria Agrícola López SL	Carlos López	654987321	carlos.lopez@maquinariaagricola.com	Carretera Sevilla-Huelva, Km 25	España	activo	t
15	Productos Químicos Andalucía	Javier Ruiz	959011223	j.ruiz@quimicosandalucia.com	Avenida de la Industria, 15	España	inactivo	f
16	Herramientas Profesionales Gómez	Manuel Gómez	607112233	manuel.gomez@herramientaspro.es	Calle Sierra Nevada, 22	España	activo	t
17	Importaciones del Norte SA	Li Wei	+86 10 6688 8888	li.wei@north-imports.cn	Chaoyang District, Jintong West Road	China	activo	f
18	Servicios Informáticos Integrales	Isabel Torres	959334455	isabel.torres@servicios-ti.es	Edificio Centro, Oficina 3A	España	activo	t
19	Mobiliario de Oficina Moderno	Peter Müller	+49 30 1234567	p.mueller@modern-office.de	Friedrichstraße 100	Alemania	activo	f
20	Textiles de Calidad Superior	Carmen Flores	678556677	carmen.flores@textilescalidad.es	Calle del Sol, 4	España	activo	t
21	Soluciones Energéticas Renovables	Jean-Pierre Dubois	+33 4 91 23 45 67	jp.dubois@energies-vertes.fr	Boulevard Longchamp, 58	Francia	inactivo	f
32	Muebles Élite	Sergio Ramos	611445566	contacto@muebleselite.com	Calle ejemplo	España	activo	f
33	Casa & Diseño	Isabel Romero	600556677	info@casaydiseno.com	Calle ejemplo	España	activo	f
22	pruebecita	pruebin	312341234	prueb@gmail.com	la prueba dolorosa	el pruebon	inactivo	t
1	asasa	Dario	3256456345	dario010904@gmail.com	avda guatemala	españa	inactivo	t
3	aaaaaaaaaaaaaa	cdd	13412341	hasdasd@gmail.com	avda pepe	España	inactivo	f
2	sisisisas	uujj	13412341	mimimi@gmail.com	la gozadera	Filipinas	inactivo	t
24	Sofás Modernos S.A.	Carlos Martín	699112233	contacto@sofasmodernos.com	Calle ejemplo	España	activo	f
25	Muebles Deluxe	Ana Ruiz	688445566	ventas@mueblesdeluxe.com	Calle ejemplo	España	activo	f
26	Diseño y Hogar	Pablo Torres	677556677	info@disenoyhogar.com	Calle ejemplo	España	activo	f
27	Confort Total	María López	666778899	ventas@conforttotal.com	Calle ejemplo	España	activo	f
14	Componentes Electrónicos Global	Sophie Dubois	+33 1 45 67 89 10	sophie.dubois@global-electronics.fr	10 Rue de la République	Francia	activo	f
36	asasaas	jimena	349889434	jimena@gmail.com	avdajimena	España	inactivo	t
37	Ejemeplito	Ejemeplito	422542354	Ejemeplito@gamicl.com	asdasdasd	44444	inactivo	t
\.


--
-- TOC entry 5051 (class 0 OID 24691)
-- Dependencies: 219
-- Data for Name: usuarios; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.usuarios (id, usuario, password, rol) FROM stdin;
1	Dario	$2a$06$ASYGxkAW6VRMK07ezfmLLOGvcc0aL4SMnNg.TAlR4yuzjsNaeJ1bS	admin
2	prueba	$2a$06$YwllLFH7o9T9UNDkeHBc5O9V0142TIO4Jr4mijL8edDDIvjSvHH.O	admin
3	empleado	$2a$06$u5o2ENj3xH8CTZsFjbeOUOBX4sEB98nF0SqDxaJfKS/08CtWaAjQG	empleado
\.


--
-- TOC entry 5067 (class 0 OID 25905)
-- Dependencies: 235
-- Data for Name: ventas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.ventas (id, fecha, id_cliente, total) FROM stdin;
4	2025-05-14 18:34:25.857513	1	599.97
5	2025-05-14 18:34:38.54614	2	369.99
2	2025-05-14 17:45:51.651454	1	2699.97
6	2025-05-14 19:02:37.778847	7	519.99
\.


--
-- TOC entry 5095 (class 0 OID 0)
-- Dependencies: 222
-- Name: clientes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.clientes_id_seq', 14, true);


--
-- TOC entry 5096 (class 0 OID 0)
-- Dependencies: 238
-- Name: compras_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.compras_id_seq', 48, true);


--
-- TOC entry 5097 (class 0 OID 0)
-- Dependencies: 226
-- Name: conversiones_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.conversiones_id_seq', 1, false);


--
-- TOC entry 5098 (class 0 OID 0)
-- Dependencies: 240
-- Name: detalle_compras_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.detalle_compras_id_seq', 47, true);


--
-- TOC entry 5099 (class 0 OID 0)
-- Dependencies: 236
-- Name: detalle_ventas_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.detalle_ventas_id_seq', 18, true);


--
-- TOC entry 5100 (class 0 OID 0)
-- Dependencies: 220
-- Name: empleados_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.empleados_id_seq', 54, true);


--
-- TOC entry 5101 (class 0 OID 0)
-- Dependencies: 228
-- Name: estadisticas_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.estadisticas_id_seq', 1, false);


--
-- TOC entry 5102 (class 0 OID 0)
-- Dependencies: 230
-- Name: horarios_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.horarios_id_seq', 24, true);


--
-- TOC entry 5103 (class 0 OID 0)
-- Dependencies: 232
-- Name: inventario_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.inventario_id_seq', 21, true);


--
-- TOC entry 5104 (class 0 OID 0)
-- Dependencies: 242
-- Name: proveedor_productos_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.proveedor_productos_id_seq', 20, true);


--
-- TOC entry 5105 (class 0 OID 0)
-- Dependencies: 224
-- Name: proveedores_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.proveedores_id_seq', 37, true);


--
-- TOC entry 5106 (class 0 OID 0)
-- Dependencies: 218
-- Name: usuarios_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.usuarios_id_seq', 1, false);


--
-- TOC entry 5107 (class 0 OID 0)
-- Dependencies: 234
-- Name: ventas_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.ventas_id_seq', 7, true);


--
-- TOC entry 4871 (class 2606 OID 24744)
-- Name: clientes clientes_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT clientes_email_key UNIQUE (email);


--
-- TOC entry 4873 (class 2606 OID 24742)
-- Name: clientes clientes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes
    ADD CONSTRAINT clientes_pkey PRIMARY KEY (id);


--
-- TOC entry 4891 (class 2606 OID 25936)
-- Name: compras compras_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.compras
    ADD CONSTRAINT compras_pkey PRIMARY KEY (id);


--
-- TOC entry 4879 (class 2606 OID 24793)
-- Name: conversiones conversiones_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.conversiones
    ADD CONSTRAINT conversiones_pkey PRIMARY KEY (id);


--
-- TOC entry 4893 (class 2606 OID 25943)
-- Name: detalle_compras detalle_compras_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_compras
    ADD CONSTRAINT detalle_compras_pkey PRIMARY KEY (id);


--
-- TOC entry 4889 (class 2606 OID 25918)
-- Name: detalle_ventas detalle_ventas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_ventas
    ADD CONSTRAINT detalle_ventas_pkey PRIMARY KEY (id);


--
-- TOC entry 4867 (class 2606 OID 24711)
-- Name: empleados empleados_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.empleados
    ADD CONSTRAINT empleados_email_key UNIQUE (email);


--
-- TOC entry 4869 (class 2606 OID 24709)
-- Name: empleados empleados_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.empleados
    ADD CONSTRAINT empleados_pkey PRIMARY KEY (id);


--
-- TOC entry 4881 (class 2606 OID 24803)
-- Name: estadisticas estadisticas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.estadisticas
    ADD CONSTRAINT estadisticas_pkey PRIMARY KEY (id);


--
-- TOC entry 4883 (class 2606 OID 24855)
-- Name: horarios horarios_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.horarios
    ADD CONSTRAINT horarios_pkey PRIMARY KEY (id);


--
-- TOC entry 4885 (class 2606 OID 25826)
-- Name: inventario inventario_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.inventario
    ADD CONSTRAINT inventario_pkey PRIMARY KEY (id);


--
-- TOC entry 4895 (class 2606 OID 25975)
-- Name: proveedor_productos proveedor_productos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedor_productos
    ADD CONSTRAINT proveedor_productos_pkey PRIMARY KEY (id);


--
-- TOC entry 4875 (class 2606 OID 24786)
-- Name: proveedores proveedores_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedores
    ADD CONSTRAINT proveedores_email_key UNIQUE (email);


--
-- TOC entry 4877 (class 2606 OID 24784)
-- Name: proveedores proveedores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedores
    ADD CONSTRAINT proveedores_pkey PRIMARY KEY (id);


--
-- TOC entry 4863 (class 2606 OID 24698)
-- Name: usuarios usuarios_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT usuarios_pkey PRIMARY KEY (id);


--
-- TOC entry 4865 (class 2606 OID 24700)
-- Name: usuarios usuarios_usuario_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT usuarios_usuario_key UNIQUE (usuario);


--
-- TOC entry 4887 (class 2606 OID 25911)
-- Name: ventas ventas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ventas
    ADD CONSTRAINT ventas_pkey PRIMARY KEY (id);


--
-- TOC entry 4902 (class 2620 OID 24702)
-- Name: usuarios encrypt_user_password; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER encrypt_user_password BEFORE INSERT OR UPDATE ON public.usuarios FOR EACH ROW EXECUTE FUNCTION public.encrypt_password();


--
-- TOC entry 4904 (class 2620 OID 25957)
-- Name: detalle_compras trg_actualizar_stock_compra; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER trg_actualizar_stock_compra AFTER INSERT ON public.detalle_compras FOR EACH ROW EXECUTE FUNCTION public.actualizar_stock_compra();


--
-- TOC entry 4903 (class 2620 OID 25955)
-- Name: detalle_ventas trg_actualizar_stock_venta; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER trg_actualizar_stock_venta AFTER INSERT ON public.detalle_ventas FOR EACH ROW EXECUTE FUNCTION public.actualizar_stock_venta();


--
-- TOC entry 4899 (class 2606 OID 25944)
-- Name: detalle_compras detalle_compras_id_compra_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_compras
    ADD CONSTRAINT detalle_compras_id_compra_fkey FOREIGN KEY (id_compra) REFERENCES public.compras(id) ON DELETE CASCADE;


--
-- TOC entry 4900 (class 2606 OID 25949)
-- Name: detalle_compras detalle_compras_id_producto_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_compras
    ADD CONSTRAINT detalle_compras_id_producto_fkey FOREIGN KEY (id_producto) REFERENCES public.inventario(id);


--
-- TOC entry 4897 (class 2606 OID 25924)
-- Name: detalle_ventas detalle_ventas_id_producto_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_ventas
    ADD CONSTRAINT detalle_ventas_id_producto_fkey FOREIGN KEY (id_producto) REFERENCES public.inventario(id);


--
-- TOC entry 4898 (class 2606 OID 25919)
-- Name: detalle_ventas detalle_ventas_id_venta_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.detalle_ventas
    ADD CONSTRAINT detalle_ventas_id_venta_fkey FOREIGN KEY (id_venta) REFERENCES public.ventas(id) ON DELETE CASCADE;


--
-- TOC entry 4896 (class 2606 OID 24856)
-- Name: horarios fk_empleado; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.horarios
    ADD CONSTRAINT fk_empleado FOREIGN KEY (empleado_id) REFERENCES public.empleados(id) ON DELETE CASCADE;


--
-- TOC entry 4901 (class 2606 OID 25976)
-- Name: proveedor_productos proveedor_productos_proveedor_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proveedor_productos
    ADD CONSTRAINT proveedor_productos_proveedor_id_fkey FOREIGN KEY (proveedor_id) REFERENCES public.proveedores(id) ON UPDATE CASCADE ON DELETE CASCADE;


-- Completed on 2025-05-19 12:11:04

--
-- PostgreSQL database dump complete
--

