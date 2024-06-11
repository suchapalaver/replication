import grpc
import replicate
import logging
import replicate_pb2
import replicate_pb2_grpc
import json
import os
import sys
from concurrent.futures import ThreadPoolExecutor

logging.basicConfig(stream=sys.stdout, level=logging.INFO)


class ReplicateServiceServicer(replicate_pb2_grpc.ReplicateServiceServicer):
    def ProcessIntent(self, request, context):
        model = request.model
        
        logging.info("Replication service received request from gRPC client ...")

        input = json.loads(request.input)

        logging.info(f"Loading Intent into {model} ...")

        output = replicate.run(
            model,
            input,
        )

        logging.info("Returning response to gRPC client ...")
        return replicate_pb2.ReplicateResponse(payload=output)


def serve():
    port = os.getenv('PORT')
    if not port:
        raise ValueError("PORT environment variable is not set.")
    logging.info(f"Starting 'replication' server. Listening on port {port}.")
    server = grpc.server(ThreadPoolExecutor(max_workers=10))
    replicate_pb2_grpc.add_ReplicateServiceServicer_to_server(ReplicateServiceServicer(), server)
    server.add_insecure_port(f"[::]:{port}")
    server.start()
    server.wait_for_termination()


if __name__ == "__main__":
    serve()
