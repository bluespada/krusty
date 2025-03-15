<script lang="ts">
    import { GitHubIcon } from 'vue3-simple-icons';
    import { defineComponent } from 'vue';
    export default defineComponent({
        name: "HeaderComponent",
        components: {
            GitHubIcon
        }
    });
</script>

<template>
    <nav class="fixed top-0 left-0 right-0 w-full z-10">
        <div class="w-full px-12 py-1.5 flex flex-row items-center justify-between">
            <div></div>
            <div class="flex flex-row items-center">
                <a href="https://github.com/bluespada/krusty" target="_blank" aria-label="Github Repositories">
                    <GitHubIcon style="fill: white" size="24"/>
                </a>
            </div>
        </div>
    </nav>
</template>