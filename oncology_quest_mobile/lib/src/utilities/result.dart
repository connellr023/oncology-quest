class Result<T> {
  final T? data;
  final String? error;

  Result.ok(this.data) : error = null;
  Result.err(this.error) : data = null;

  bool get isOk => data != null;
  bool get isErr => error != null;
}