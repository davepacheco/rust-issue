/*!
 * Hello
 *
 * * Hello
 *     * The HTTP layer of the server may generate an error.  In this case, it
 *       may be just as easy to generate the appropriate HTTP response (with a
 *       400-level or 500-level status code) as it would be to generate an Error
 *       object of some kind.
 *
 * To achive this, we first define `HttpError`, which provides a status code,
 * error code (via an Enum), external message (for sending in the response),
 * optional metadata, and an internal message (for the log file or other
 * instrumentation).  The HTTP layers of the request-handling stack may use this
 * struct directly.  **The set of possible error codes here is part of the
 * OpenAPI contract, as is the schema for any metadata.**  By the time an error
 * bubbles up to the top of the request handling stack, it must be an
 * HttpError.
 */
