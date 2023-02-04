<script setup lang="ts">
import { ref, onMounted } from "vue";
import gsap from "gsap";

const errorTagRef = ref();

// gsap animation
onMounted(() => {
	gsap.fromTo(
		errorTagRef.value,
		{
			x: -100,
		},
		{
			x: 0,
			duration: 0.5,
		}
	);
});

// set error message
const props = defineProps<{
	message: string;
}>();
</script>

<template>
	<div ref="errorTagRef" class="error-message-container">
		<div class="flex justify-center items-center basis-1/4">
            <!-- error icon in svg -->
			<svg
				xmlns="http://www.w3.org/2000/svg"
				xmlns:xlink="http://www.w3.org/1999/xlink"
				version="1.1"
				width="25"
				height="25"
				viewBox="0 0 256 256"
				xml:space="preserve"
			>
				<defs></defs>
				<g
					style="
						stroke: none;
						stroke-width: 0;
						stroke-dasharray: none;
						stroke-linecap: butt;
						stroke-linejoin: miter;
						stroke-miterlimit: 10;
						fill: none;
						fill-rule: nonzero;
						opacity: 1;
					"
					transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
				>
					<path
						d="M 45 90 C 20.187 90 0 69.813 0 45 C 0 20.187 20.187 0 45 0 c 24.813 0 45 20.187 45 45 C 90 69.813 69.813 90 45 90 z"
						style="
							stroke: none;
							stroke-width: 1;
							stroke-dasharray: none;
							stroke-linecap: butt;
							stroke-linejoin: miter;
							stroke-miterlimit: 10;
							fill: rgb(239, 35, 60);
							fill-rule: nonzero;
							opacity: 1;
						"
						transform=" matrix(1 0 0 1 0 0) "
						stroke-linecap="round"
					/>
					<path
						d="M 28.902 66.098 c -1.28 0 -2.559 -0.488 -3.536 -1.465 c -1.953 -1.952 -1.953 -5.118 0 -7.07 l 32.196 -32.196 c 1.951 -1.952 5.119 -1.952 7.07 0 c 1.953 1.953 1.953 5.119 0 7.071 L 32.438 64.633 C 31.461 65.609 30.182 66.098 28.902 66.098 z"
						style="
							stroke: none;
							stroke-width: 1;
							stroke-dasharray: none;
							stroke-linecap: butt;
							stroke-linejoin: miter;
							stroke-miterlimit: 10;
							fill: rgb(255, 255, 255);
							fill-rule: nonzero;
							opacity: 1;
						"
						transform=" matrix(1 0 0 1 0 0) "
						stroke-linecap="round"
					/>
					<path
						d="M 61.098 66.098 c -1.279 0 -2.56 -0.488 -3.535 -1.465 L 25.367 32.438 c -1.953 -1.953 -1.953 -5.119 0 -7.071 c 1.953 -1.952 5.118 -1.952 7.071 0 l 32.195 32.196 c 1.953 1.952 1.953 5.118 0 7.07 C 63.657 65.609 62.377 66.098 61.098 66.098 z"
						style="
							stroke: none;
							stroke-width: 1;
							stroke-dasharray: none;
							stroke-linecap: butt;
							stroke-linejoin: miter;
							stroke-miterlimit: 10;
							fill: rgb(255, 255, 255);
							fill-rule: nonzero;
							opacity: 1;
						"
						transform=" matrix(1 0 0 1 0 0) "
						stroke-linecap="round"
					/>
				</g>
			</svg>
		</div>
		<div class="basis-3/4 flex items-center text-lg">
			<h1 class="error-message">{{ props.message }}</h1>
		</div>
	</div>
</template>

<style>
.error-message-container {
	height: 5vh;
	border-radius: 0.3em;
	background: rgb(103, 0, 0);
	background: linear-gradient(
		90deg,
		rgba(103, 0, 0, 1) 0%,
		rgba(27, 39, 52, 1) 40%,
		rgba(var(--background-color-friend-hex), 1) 100%
	);

	display: flex;
	flex-direction: row;
}

.error-message {
	color: #ef233c;
}

</style>
