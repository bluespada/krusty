
<script lang="ts">
    import { defineComponent, ref } from 'vue';
    import HeaderComponent from './components/HeaderComponent.vue';
    import CounterComponent from './components/CounterComponent.vue';
    import FooterComponent from './components/FooterComponent.vue';

    export default defineComponent({
        name: "App",
        components: {
            HeaderComponent,
            CounterComponent,
            FooterComponent,
        },
    });
</script>

<template>
    <HeaderComponent/>
    <main class="w-full h-full min-h-screen flex flex-col items-center justify-center gap-2.5 font-os">
        <h1 class="text-3xl">Yey Welcome To Krusty</h1>
        <span class="text-gray-300 text-sm">Edit <code class="font-jb bg-black">./resources/src/App.vue</code> to test HMR.</span>
        <CounterComponent/>
    </main>
    <FooterComponent/>
</template>