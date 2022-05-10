import numpy as np
import pymannkendall as mk
from scipy.stats import norm
# Data generation for analysis
data = [202.175842,222.266602,250.772827,300.741455,350.643982,369.749268,400.998474,479.279663,486.617310,487.517456,491.321625]

result = mk.original_test(data)
print(result)
# Mann_Kendall_Test(trend='increasing', h=True, p=2.623614939456509e-05, z=4.203894298472224, Tau=1.0, s=55.0, var_s=165.0, slope=33.419006333333336, intercept=202.6542363333333)

print(norm.ppf(0.975))