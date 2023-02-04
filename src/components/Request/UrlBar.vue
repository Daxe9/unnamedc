<script setup lang="ts">
import { ref } from "vue";

const urlInput = ref<string>("");

// check opearting mode
if (import.meta.env.MODE === "development") {
	urlInput.value = "http://localhost:3000/";
}

const emits = defineEmits(["urlInput"]);

// make urlInput accessible from parent element
defineExpose({
	urlInput,
});

// emit urlInput to parent element
function sendUrl(e: any) {
	e.preventDefault();
	emits("urlInput");
}
</script>

<template>
	<form class="url-bar" @submit="sendUrl">
		<input type="text" v-model="urlInput" placeholder="URL" />
		<button class="bar-confirm px-4" @click="sendUrl">Send</button>
	</form>
</template>

<style>
input[type="text"] {
	background-color: transparent;
	color: var(--light-color);
	border: 1px solid var(--light-color);
	width: 100%;
	outline: none;
	padding: 0 0.2em;
}

.url-bar {
	width: 98%;
	height: 6.5%;
	max-height: 35px;

	margin: 0.5em auto 0 auto;
	border: 1px solid black;

	display: flex;
}

.bar-confirm {
	max-width: 20%;
	border: 1px solid var(--light-color);
    transition: all 0.4s ease-in-out;
}

.bar-confirm:hover {
	box-shadow: 1px 0 2px 1px;
	background-color: var(--light-color);
	color: var(--dark-color);
}
</style>
