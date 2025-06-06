import { createRouter, createWebHistory } from 'vue-router';
import Login from '../views/Login.vue';
import Home from '../views/Home.vue';
import Empleados from '../views/empleados/index.vue';
import Inventario from '../views/inventario/index.vue';
import Clientes from '../views/clientes/index.vue';
import Ordenes from '../views/ordenes/index.vue';
import Proveedores from '../views/proveedores/index.vue';
import Conversiones from '../views/Utilidades/Utilidades.vue';
import Estadisticas from '../views/estadisticas/Dashboard.vue';

const routes = [
  { path: '/', component: Login },
  { path: '/home', name: 'home', component: Home },
  { path: '/empleados', name: 'empleados', component: Empleados, meta: { requiresAdmin: true } },
  { path: '/inventario', name: 'inventario', component: Inventario },
  { path: '/clientes', name: 'clientes', component: Clientes },
  { path: '/ordenes', name: 'ordenes', component: Ordenes },
  { path: '/proveedores', name: 'proveedores', component: Proveedores },
  { path: '/conversiones', name: 'conversiones', component: Conversiones },
  { path: '/estadisticas', name: 'estadisticas', component: Estadisticas, meta: { requiresAdmin: true } },

  {
    path: '/estadisticas/clientes',
    component: () => import('../views/estadisticas/components/clientes.vue'),
  },
  {
    path: '/estadisticas/ventas',
    component: () => import('../views/estadisticas/components/ventas.vue'),
  },
   {
    path: '/estadisticas/empleados',
    component: () => import('../views/estadisticas/components/empleados.vue'),
  },
  {
    path: '/estadisticas/horarios',
    component: () => import('../views/estadisticas/components/horarios.vue'),
  },
  {
    path: '/estadisticas/inventario',
    component: () => import('../views/estadisticas/components/inventario.vue'),
  },
   {
    path: '/estadisticas/proveedores',
    component: () => import('../views/estadisticas/components/proveedores.vue'),
  },

   {
    path: '/estadisticas/compras',
    component: () => import('../views/estadisticas/components/compras.vue'),
  },

];

const router = createRouter({
  history: createWebHistory(),
  routes
});

// Protección de rutas según el rol del usuario
router.beforeEach((to, from, next) => {
  const userStr = localStorage.getItem('user');
  const user = userStr ? JSON.parse(userStr) : null;
  const rol = user ? user.rol : null;

  if (to.path !== '/' && !rol) {
    next('/'); // Redirect to login if not authenticated
  } else if (to.meta.requiresAdmin && rol !== 'admin') {
    next('/home'); // Redirect non-admin users
  } else {
    next();
  }
});

export default router;
