import Vue from 'vue';
import App from './App';

import 'bootstrap';
import './assets/styles/style.scss';

new Vue({
    el: '#app',
    template: '<App/>',
    components: { App },
    render: h => h(App)
})
