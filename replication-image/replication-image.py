import grpc
import replicate
import logging
import image_pb2
import image_pb2_grpc
import json
import os
import sys
from concurrent.futures import ThreadPoolExecutor

logging.basicConfig(stream=sys.stdout, level=logging.INFO)


class ImageServiceServicer(image_pb2_grpc.ImageServiceServicer):
    def ProcessIntent(self, request, context):
        logging.info("Image service received request from gRPC client ...")

        model = request.model
        input = json.loads(request.input)

        logging.info(f"Loading Intent into {model} ...")

        output = replicate.run(
            model,
            input,
        )

        logging.info("Returning response to gRPC client ...")
        return image_pb2.ImageResponse(img_urls=output)


def serve():
    port = os.getenv('PORT')
    if not port:
        raise ValueError("PORT environment variable is not set.")
    logging.info(f"Starting 'replication-image' server. Listening on port {port}.")
    server = grpc.server(ThreadPoolExecutor(max_workers=10))
    image_pb2_grpc.add_ImageServiceServicer_to_server(ImageServiceServicer(), server)
    server.add_insecure_port(f"[::]:{port}")
    server.start()
    server.wait_for_termination()


if __name__ == "__main__":
    serve()
