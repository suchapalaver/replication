FROM python:3.9-slim as builder

WORKDIR /app

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc

RUN python -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

COPY requirements.txt .
RUN pip install --upgrade pip && \
    pip install -r requirements.txt

COPY replicate.proto .
RUN python3 -m grpc_tools.protoc -I . --python_out=. --grpc_python_out=. replicate.proto

FROM python:3.9-slim

COPY --from=builder /opt/venv /opt/venv
COPY --from=builder /app /app

WORKDIR /app

COPY . /app

ENV PATH="/opt/venv/bin:$PATH"

ENV REPLICATE_API_TOKEN=.env

ARG PORT

ENV PORT=${PORT}

CMD ["python", "replication.py"]
