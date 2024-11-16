let titles_type = ["Dataset", "Survey", "Review", "Overview"];
let titles_model = ["manipulation", "locomotion", "Task Planning", "Motion Planning", "Long-term", "Planning", "Optimization", "Estimation", "Control", "quadruped", "legged robot"];
let titles_method = ["bayesian", "task", "manipulation", "Navigation", "Decision", "Reinforcement", "dynamic", "sparsification", "Guassian", "differentiable", "simulation", "simulator", "dataset", "Deformable", 
"geometric", "benchmark", "evaluation", "prediction", "non prehensile", "grasp", "3DGS", "POMDP", "MDP", "trajectory", "trajectories", "GPT", "LLM", "embodied", "VLM"];
let titles =  titles_model + titles_method + titles_type;
let authors_array = [
    "Yuke Zhu", "Leslie Kaelbling", "Panpan CAI", "David Hsu",
    "Wolfram Burgard", "Yunzhu Li", "Ken Goldberg", "Guanya Shi",
    "Marco Pavone", "Chelsea Finn", "Shuran Song", "Danfei Xu",
    "Marco Hutter", "Xiaolong Wang", "Li Fei-Fei", "Pieter Abbeel",
    "Shaojie Shen", "Tong Qin", "Hongyang Li", "Junchi Yan", "Karl Pertsch", "Boris Ivanovic", "Yuke Zhu",
    "Ken Goldenberg", "Deepak Pathek", "Sergey Levine", "Cewu Lu"];
    

let conferences = [
    "AAAI", "CORL", "RAS", "CoRL",
    "RA-L", "RAL", "IJRR", "ICRA", "IJCAI", "IROS",
    "UAI", "RSS", "TRO",
    "CVPR", "ECCV", "ICCV", "Transactions on Robotics",
    "International Journal of Robotics Research", "Robotics and Automation Letters", "ICLR", "ICML",
    "NeurIPS", "NIPS", "COLT"];

let start = conferences.shift();
let conferences_rg = conferences.reduce(|sum, v| sum + "|" + v, start);
let conferences_rg = "(" + conferences_rg + ")([\\s'-]*\\d+)?";

let start = titles.shift();
let title_rg = titles.reduce(|sum, v| sum + "|" + v, start);

let start = authors_array.shift();
let authors_rg = authors_array.reduce(|sum, v| sum + "|" + v, start);

export title_rg;
export authors_rg;
export conferences_rg;
