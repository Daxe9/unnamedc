<script setup lang="ts">
import { ref } from "vue";

const urlInput = ref<string>("");

if (import.meta.env.MODE === "development") {
	urlInput.value = "http://localhost:3000/";
}

const emits = defineEmits(["urlInput"]);

defineExpose({
	urlInput,
});

function sendUrl(e: any) {
	e.preventDefault();
	emits("urlInput");
}
</script>

<template>
	<form class="bar" @submit="sendUrl">
		<input type="text" v-model="urlInput" placeholder="URL" />
		<button class="bar-confirm" @click="sendUrl">Send</button>
	</form>
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
