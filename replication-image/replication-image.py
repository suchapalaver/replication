import grpc
import replicate
import logging
import image_pb2
import image_pb2_grpc
import sys
from concurrent.futures import ThreadPoolExecutor

logging.basicConfig(stream=sys.stdout, level=logging.INFO)


class ImageServiceServicer(image_pb2_grpc.ImageServiceServicer):
    def ProcessIntent(self, request, context):
        logging.info("Image service received request from gRPC client ...")

        intent = request.intent

        logging.info("Loading Intent into 'stability-ai/sdxl' ...")

        output = replicate.run(
            "stability-ai/sdxl:7762fd07cf82c948538e41f63f77d685e02b063e37e496e96eefd46c929f9bdc",
            input={
                "width": 768,
                "height": 768,
                "prompt": intent,
                "refine": "expert_ensemble_refiner",
                "scheduler": "K_EULER",
                "lora_scale": 0.6,
                "num_outputs": 1,
                "guidance_scale": 7.5,
                "apply_watermark": False,
                "high_noise_frac": 0.8,
                "negative_prompt": "",
                "prompt_strength": 0.8,
                "num_inference_steps": 25,
            },
        )

        logging.info("Returning response to gRPC client ...")
        return image_pb2.ImageResponse(img_urls=output)


def serve():
    logging.info("Starting 'replication-image' server. Listening on port 50051.")
    server = grpc.server(ThreadPoolExecutor(max_workers=10))
    image_pb2_grpc.add_ImageServiceServicer_to_server(
        ImageServiceServicer(), server
    )
    server.add_insecure_port("[::]:50051")
    server.start()
    server.wait_for_termination()


if __name__ == "__main__":
    serve()
