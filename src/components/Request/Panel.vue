<script setup lang="ts">
import { onMounted, ref } from "vue";

const settings = ref<Array<HTMLElement>>([]);

onMounted( async () => {
    for (let s of settings.value) {
        console.log(s)
    }
})
</script>

<template>
    <div>
        <div class="preview">

        </div>

        <div class="settings">
            <div ref="settings" class="single-setting">
                <h3>Title1</h3>
                <div class="dropdown">
                    <ul>
                        <li>Item 1</li>
                        <li>Item 2</li>
                        <li>Item 3</li>
                    </ul>
                </div>
            </div>

            <div ref="settings" class="single-setting">
                <h3>Title2</h3>
                <div class="dropdown">
                    <ul>
                        <li>Item 4</li>
                        <li>Item 5</li>
                        <li>Item 6</li>
                    </ul>
                </div>
            </div>
        </div> 
    </div>
</template>

<style scoped>
    
</style>
