<script setup lang="ts">
import Request from "@/components/Request.vue";
import Response from "@/components/Response.vue";
</script>
<template>
	<div class="main-container">
		<Request />
		<Response />
	</div>
</template>

<style scoped>
.main-container {
	display: flex;
	flex-direction: column;
	width: 100%;
	height: 100%;
	color: #ffffff;
}

.main-container > * {
	flex-grow: 1;
}
</style>
