<script setup lang="ts">
import { ref } from "vue";
import UrlBar from "./UrlBar.vue";

const urlBarRef = ref();

function getUrlInput() {
	return urlBarRef.value.urlInput;
}
</script>
<template>
	<div class="request-content">
		<UrlBar ref="urlBarRef" />
        <div class="content">
            <div class="meta">
            </div>

            <div class="body">
            </div>
        </div>
	</div>
</template>

<style scoped>
.request-content {
	background-color: #242238;
}
</style>
