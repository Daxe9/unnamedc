<script setup lang="ts">
import { ref } from "vue";

const urlInput = ref<string>("");

defineExpose({
	urlInput,
});
</script>

<template>
<div class="bar">
		<input type="text" v-model="urlInput" />
        </div>
</template>

<style>
@import url("../variables.css");

	/* background-color: var(--url-bar-background-color); */
input[type="text"] {
	background-color: var(--url-bar-background-color);
	color: var(--url-bar-text-color);
	width: 98%;
    height: 100%;
	outline: none;
	border: none;
	border-radius: 1em;
    margin: 0 auto;

}

.bar {
    width: 100%;
    height: 6%;
display: flex;
         jusjustify-content: center;
}
</style>
