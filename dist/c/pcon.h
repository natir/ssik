/* c bindings to the pcon */

#ifndef _PCON_HEADER_GUARD_
#define _PCON_HEADER_GUARD_

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Error emmit when pcon try to work with file
 */
typedef enum {
  /**
   * We can't create file. In C binding it's equal to 0
   */
  CantCreateFile,
  /**
   * We can't open file. In C binding it's equal to 1
   */
  CantOpenFile,
  /**
   * Error durring write in file. In C binding it's equal to 2
   */
  ErrorDurringWrite,
  /**
   * Error durring read file. In C binding it's equal to 3
   */
  ErrorDurringRead,
  /**
   * No error, this exist only for C binding it's the value of a new error pointer
   */
  NoError,
} IO;

/**
 * A counter of kmer based on cocktail crate 2bit conversion, canonicalisation and hashing.
 * If kmer occure more than 256 other occurence are ignored
 */
typedef struct Counter Counter;

/**
 * A struct to store if a kmer is Solid or not. Only kmer with abundance upper than a threshold is solid
 */
typedef struct Solid Solid;

typedef uint8_t Count;

/**
 * Perform count of kmer in fasta file in path, this file can be compress in gzip, bzip2, xz.
 * You must check value of `io_error` is equal to NoError before use `counter`.
 *
 * In Python it's count_fasta method of Counter object.
 * See [counter::Counter::count_fasta].
 */
void pcon_counter_count_fasta(Counter *counter,
                              const char *c_path,
                              uintptr_t read_buffer_len,
                              IO *io_error);

/**
 * Free a Counter. In Python use del on Counter object.
 *
 * # Safety
 * It's safe
 */
void pcon_counter_free(Counter *counter);

/**
 * Get the count of value `kmer`
 *
 * In Python it's get method of Counter object.
 * See [counter::Counter::get].
 */
Count pcon_counter_get(const Counter *counter, uint64_t kmer);

/**
 * Get the count of value a canonical `kmer`
 *
 * In Python it's get_canonic method of Counter object.
 * See [counter::Counter::get_canonic].
 */
Count pcon_counter_get_canonic(const Counter *counter, uint64_t kmer);

/**
 * Increase the count of `kmer`
 *
 * In Python it's inc method of Counter object.
 * See [counter::Counter::inc].
 */
void pcon_counter_inc(Counter *counter, uint64_t kmer);

/**
 * Increase the count of a canonical `kmer`
 *
 * In Python it's inc_canonic method of Counter object.
 * See [counter::Counter::inc_canonic].
 */
void pcon_counter_inc_canonic(Counter *counter, uint64_t kmer);

/**
 * Create a new Counter. In python binding Counter is an object, new is the default constructor.
 * See [counter::Counter::new].
 */
Counter *pcon_counter_new(uint8_t k);

/**
 * Deserialize Counter from `c_path` in `counter`
 * You must check value of `io_error` is equal to NoError before use `counter`
 *
 * In Python it's deserialize class method of Counter.
 * See [counter::Counter::deserialize].
 */
void pcon_deserialize_counter(Counter *counter, const char *c_path, IO *io_error);

/**
 * Deserialize Solid from `c_path` in `counter`
 * You must check value of `io_error` is equal to NoError before use `solid`
 *
 * In Python it's deserialize class method of solid.
 * See [solid::Solid::deserialize].
 */
void pcon_deserialize_solid(Solid *solid, const char *c_path, IO *io_error);

/**
 * See [dump::csv].
 * You must check value of `io_error` is equal to NoError to be sure no problem occure durring write
 *
 * In Python it's csv function of dump module.
 */
void pcon_dump_csv(const Counter *counter, Count abundance, const char *c_path, IO *io_error);

/**
 * See [dump::solid()].
 * You must check value of `io_error` is equal to NoError to be sure no problem occure durring write
 *
 * In Python it's solid function of dump module.
 */
void pcon_dump_solid(const Counter *counter, Count abundance, const char *c_path, IO *io_error);

/**
 * See [dump::spectrum].
 * You must check value of `io_error` is equal to NoError to be sure no problem occure durring write
 *
 * In Python it's spectrum function of dump module.
 */
void pcon_dump_spectrum(const Counter *counter, const char *c_path, IO *io_error);

/**
 * Free a pcon io error
 *
 * # Safety
 * It's safe
 */
void pcon_error_free(IO *error);

/**
 * Create a new pcon io error it's init to no error, see [error::IO]. In python corresponding string error is emit.
 */
IO *pcon_error_new(void);

/**
 * Serialize Counter in path of file
 * You must check value of `io_error` is equal to NoError before use `counter`
 *
 * In Python it's serialize method of Counter object.
 * See [counter::Counter::serialize].
 */
void pcon_serialize_counter(const Counter *counter, const char *c_path, IO *io_error);

/**
 * Serialize Solid in path of file
 * You must check value of `io_error` is equal to NoError before use `solid`
 *
 * In Python it's serialize method of Solid object.
 * See [solid::Solid::serialize].
 */
void pcon_serialize_solid(const Solid *solid, const char *c_path, IO *io_error);

/**
 * See [set_count_nb_threads]
 */
void pcon_set_nb_threads(uintptr_t nb_threads);

/**
 * Free a Solid. In Python use del on Solid object.
 *
 * # Safety
 * It's safe
 */
void pcon_solid_free(Solid *solid);

/**
 * Create a new Solid from value in Counter
 * In python binding, this is a Solid class method from_counter.
 * See [solid::Solid::from_counter].
 */
Solid *pcon_solid_from_counter(const Counter *counter, Count abundance);

/**
 * Get the solidity status of `kmer`
 *
 * In Python it's get method of Solid object.
 * See [solid::Solid::get].
 */
bool pcon_solid_get(Solid *solid, uint64_t kmer);

/**
 * Get the solidity status of a canonical `kmer`
 *
 * In Python it's get_canonic method of Solid object.
 * See [solid::Solid::get_canonic].
 */
bool pcon_solid_get_canonic(Solid *solid, uint64_t kmer);

/**
 * Create a new Solid. In python binding Solid is an object, new is the default constructor.
 * See [solid::Solid::new]
 */
Solid *pcon_solid_new(uint8_t k);

/**
 * Set the solidity status of `kmer` to `value`
 *
 * In Python it's set method of Solid object.
 * See [solid::Solid::set].
 */
void pcon_solid_set(Solid *solid, uint64_t kmer, bool value);

/**
 * Set the solidity status of a canonical `kmer` to `value`
 *
 * In Python it's set_canonic method of Solid object.
 * See [solid::Solid::set_canonic].
 */
void pcon_solid_set_canonic(Solid *solid, uint64_t kmer, bool value);

#endif /* _PCON_HEADER_GUARD_ */
