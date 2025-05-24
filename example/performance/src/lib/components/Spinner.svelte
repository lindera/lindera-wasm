<div class="spinner-overlay">
	<div class="spinner"></div>
</div>

<style>
	.spinner-overlay {
		width: 100%;
		height: 100%;
		background-color: rgba(255, 255, 255, 0.7); /* 半透明の背景 */
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 6px;
	}

	.spinner {
		border: 4px solid rgba(0, 0, 0, 0.1);
		border-top: 4px solid #3498db;
		border-radius: 50%;
		width: 30px;
		height: 30px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
