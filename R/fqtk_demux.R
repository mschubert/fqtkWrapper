#' Run `fqtk demux` with exit-code checking
#'
#' This wrapper calls the Rust-backed CLI binding and stops on non-zero
#' exit codes while streaming stdout/stderr from the tool.
#'
#' @inheritParams fqtk_demux_internal
#'
#' @export
fqtk_demux <- function(inputs, max_mismatches, read_structures, sample_metadata, output) {
  exit_code <- fqtk_demux_internal(
    path.expand(inputs),
    max_mismatches,
    read_structures,
    path.expand(sample_metadata),
    path.expand(output)
  )
  if (!isTRUE(exit_code == 0L)) {
    stop("fqtk demux failed (see stdout/stderr above).", call. = FALSE)
  }
}
