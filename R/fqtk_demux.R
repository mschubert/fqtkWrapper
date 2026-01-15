#' Run `fqtk demux` with exit-code checking
#'
#' This wrapper calls the Rust-backed CLI binding and stops on non-zero
#' exit codes while relaying stdout/stderr as R messages.
#'
#' @inheritParams fqtk_demux_internal
#' @param verbose Logical; when TRUE (default), relay stdout/stderr as messages.
#' @export
fqtk_demux <- function(inputs, max_mismatches, read_structures, sample_metadata, output, verbose = TRUE) {
  call_fun <- function() {
    fqtk_demux_internal(
      inputs = path.expand(inputs),
      max_mismatches = max_mismatches,
      read_structures = read_structures,
      sample_metadata = path.expand(sample_metadata),
      output = path.expand(output)
    )
  }

  status <- if (isTRUE(verbose)) call_fun() else suppressMessages(call_fun())
  if (status != 0L) {
    stop(paste("fqtk demux failed with status:", status))
  }
}
