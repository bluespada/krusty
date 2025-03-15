<script lang="ts">
    import { defineComponent, ref } from 'vue';
    export default defineComponent({
        name: "CounterComponent",
        setup(){
            const count = ref(0);
            const increment = () => count.value++;
            const decrement = () => count.value--;
            return { count, increment, decrement};
        }
    });
</script>

<template>
    <div class="flex flex-row gap-3 items-center">
        <button @click="decrement" class="!bg-primary px-4 py-1.5 rounded-md cursor-pointer" aria-label="Decrease">-</button>
        <span>Count {{ count }}</span>
        <button class="!bg-primary px-4 py-1.5 rounded-md cursor-pointer" aria-label="Increase" @click="increment">+</button>
    </div>
</template>