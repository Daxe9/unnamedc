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
@import url("../main.css");

input[type="text"] {
	background-color: var(--url-bar-background-color);
	color: var(--url-bar-text-color);
	width: 98%;
	height: 100%;
	outline: none;
	border: none;
	margin: 0 auto;
	padding: 0 0.2em;
}

.bar {
	width: 100%;
	height: 6%;
	display: flex;
}

.bar-confirm {
	outline: none;
	border: none;
	background-color: var(--url-bar-button-color);
	padding: 0 1em;
}
</style>
