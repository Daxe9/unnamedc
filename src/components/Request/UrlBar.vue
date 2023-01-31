<script setup lang="ts">
import { ref } from "vue";

const urlInput = ref<string>("");

const emits = defineEmits(["urlInput"]);

defineExpose({
	urlInput,
});

function sendUrl() {
	emits("urlInput");
}
</script>

<template>
	<div class="bar">
		<input type="text" v-model="urlInput" placeholder="URL" />
		<button class="bar-confirm" @click="sendUrl">Send</button>
	</div>
</template>

<style>

input[type="text"] {
	background-color: var(--url-bar-background-color);
	color: var(--url-bar-text-color);
	width: 100%;
	outline: none;
	border: none;
	margin: 0 auto;
	padding: 0 0.2em;
}

.bar {
	width: 95%;
    height: 6.5%;
	max-height: 35px;

    margin: 0.5em auto 0 auto;
    border: 1px solid black;
    
	display: flex;
}

.bar-confirm {
	outline: none;
	border: none;
	background-color: var(--url-bar-button-color);
	padding: 0 1em;
}
</style>
