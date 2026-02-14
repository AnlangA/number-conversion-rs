"""
SymPy-based calculator engine for Rust integration via PyO3.
Provides expression evaluation with support for mathematical functions and constants.
"""

import sympy as sp
from sympy import (
    sin, cos, tan, asin, acos, atan, sinh, cosh, tanh,
    log, ln, sqrt, Abs, floor, ceiling,
    exp, Pow, Min, Max,
    pi, E, oo, zoo, nan,
    sympify, SympifyError
)
import json
import sys


def evaluate_expression(expr_str: str) -> dict:
    """
    Evaluate a mathematical expression string using SymPy.
    
    Args:
        expr_str: Mathematical expression in decimal notation
        
    Returns:
        dict with keys:
        - 'success': bool
        - 'value': float (if success)
        - 'error': str (if not success)
    """
    try:
        # Clean the expression
        expr_str = expr_str.strip()
        if not expr_str:
            return {'success': False, 'error': 'Empty expression'}
        
        # Replace common notation
        expr_str = expr_str.replace('^', '**')  # Power operator
        
        # Define local namespace with math functions and constants
        local_dict = {
            # Trigonometric
            'sin': sin, 'cos': cos, 'tan': tan,
            'asin': asin, 'acos': acos, 'atan': atan,
            'sinh': sinh, 'cosh': cosh, 'tanh': tanh,
            # Logarithmic
            'log': log, 'ln': ln,
            # Other functions
            'sqrt': sqrt, 'abs': Abs,
            'floor': floor, 'ceil': ceiling, 'ceiling': ceiling,
            'round': lambda x: round(float(x)),
            'exp': exp,
            'pow': lambda x, y: Pow(x, y),
            'min': Min, 'max': Max,
            # Constants
            'pi': pi, 'PI': pi,
            'e': E, 'E': E,
        }
        
        # Parse the expression
        expr = sympify(expr_str, locals=local_dict)
        
        # Evaluate to a numeric value
        result = expr.evalf()
        
        # Check for special values
        if result == oo:
            return {'success': False, 'error': 'Result is infinity'}
        if result == zoo:
            return {'success': False, 'error': 'Result is complex infinity'}
        if result == nan:
            return {'success': False, 'error': 'Result is not a number'}
        
        # Convert to float
        float_val = float(result)
        
        # Check if it's finite
        if not (float_val == float_val):  # NaN check
            return {'success': False, 'error': 'Result is not a number'}
        if abs(float_val) > 1e308:
            return {'success': False, 'error': 'Result overflow'}
            
        return {'success': True, 'value': float_val}
        
    except SympifyError as e:
        return {'success': False, 'error': f'Syntax error: {str(e)}'}
    except ZeroDivisionError:
        return {'success': False, 'error': 'Division by zero'}
    except ValueError as e:
        return {'success': False, 'error': f'Value error: {str(e)}'}
    except TypeError as e:
        return {'success': False, 'error': f'Type error: {str(e)}'}
    except Exception as e:
        return {'success': False, 'error': f'Evaluation error: {str(e)}'}


def main():
    """CLI interface for testing."""
    if len(sys.argv) > 1:
        expr = ' '.join(sys.argv[1:])
    else:
        expr = sys.stdin.read().strip()
    
    result = evaluate_expression(expr)
    print(json.dumps(result))


if __name__ == '__main__':
    main()
